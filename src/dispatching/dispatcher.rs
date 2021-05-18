use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, DispatcherHandler, UpdateWithCx,
    },
    error_handlers::{ErrorHandler, LoggingErrorHandler},
};
use core::panic;
use futures::{Future, StreamExt};
use std::{
    fmt::Debug,
    sync::{
        atomic::{AtomicU8, Ordering},
        Arc,
    },
    time::Duration,
};
use teloxide_core::{
    requests::Requester,
    types::{
        CallbackQuery, ChatMemberUpdated, ChosenInlineResult, InlineQuery, Message, Poll,
        PollAnswer, PreCheckoutQuery, ShippingQuery, Update, UpdateKind,
    },
};
use tokio::{
    sync::{mpsc, Notify},
    time::timeout,
};

type Tx<Upd, R> = Option<mpsc::UnboundedSender<UpdateWithCx<Upd, R>>>;

#[macro_use]
mod macros {
    /// Pushes an update to a queue.
    macro_rules! send {
        ($requester:expr, $tx:expr, $update:expr, $variant:expr) => {
            send($requester, $tx, $update, stringify!($variant));
        };
    }
}

fn send<'a, R, Upd>(requester: &'a R, tx: &'a Tx<R, Upd>, update: Upd, variant: &'static str)
where
    Upd: Debug,
    R: Requester + Clone,
{
    if let Some(tx) = tx {
        if let Err(error) = tx.send(UpdateWithCx { requester: requester.clone(), update }) {
            log::error!(
                "The RX part of the {} channel is closed, but an update is received.\nError:{}\n",
                variant,
                error
            );
        }
    }
}

/// One dispatcher to rule them all.
///
/// See the [module-level documentation](crate::dispatching) for the design
/// overview.
pub struct Dispatcher<R> {
    requester: R,

    messages_queue: Tx<R, Message>,
    edited_messages_queue: Tx<R, Message>,
    channel_posts_queue: Tx<R, Message>,
    edited_channel_posts_queue: Tx<R, Message>,
    inline_queries_queue: Tx<R, InlineQuery>,
    chosen_inline_results_queue: Tx<R, ChosenInlineResult>,
    callback_queries_queue: Tx<R, CallbackQuery>,
    shipping_queries_queue: Tx<R, ShippingQuery>,
    pre_checkout_queries_queue: Tx<R, PreCheckoutQuery>,
    polls_queue: Tx<R, Poll>,
    poll_answers_queue: Tx<R, PollAnswer>,
    my_chat_members_queue: Tx<R, ChatMemberUpdated>,
    chat_members_queue: Tx<R, ChatMemberUpdated>,

    shutdown_state: Arc<AtomicShutdownState>,
    shutdown_notify_back: Notify,
}

impl<R> Dispatcher<R>
where
    R: Send + 'static,
{
    /// Constructs a new dispatcher with the specified `requester`.
    #[must_use]
    pub fn new(requester: R) -> Self {
        Self {
            requester,
            messages_queue: None,
            edited_messages_queue: None,
            channel_posts_queue: None,
            edited_channel_posts_queue: None,
            inline_queries_queue: None,
            chosen_inline_results_queue: None,
            callback_queries_queue: None,
            shipping_queries_queue: None,
            pre_checkout_queries_queue: None,
            polls_queue: None,
            poll_answers_queue: None,
            my_chat_members_queue: None,
            chat_members_queue: None,
            shutdown_state: Arc::new(AtomicShutdownState {
                inner: AtomicU8::new(ShutdownState::IsntRunning as _),
            }),
            shutdown_notify_back: Notify::new(),
        }
    }

    #[must_use]
    #[allow(clippy::unnecessary_wraps)]
    fn new_tx<H, Upd>(&self, h: H) -> Tx<R, Upd>
    where
        H: DispatcherHandler<R, Upd> + Send + 'static,
        Upd: Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        tokio::spawn(async move {
            let fut = h.handle(rx);
            fut.await;
        });
        Some(tx)
    }

    /// Setup `^C` handler which [`shutdown`]s dispatching.
    ///
    /// [`shutdown`]: Dispatcher::shutdown
    #[cfg(feature = "ctrlc_handler")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ctrlc_handler")))]
    pub fn setup_ctrlc_handler(self) -> Self {
        let shutdown_state = Arc::clone(&self.shutdown_state);
        tokio::spawn(async move {
            loop {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ^C");

                // If dispatcher wasn't running, then there is nothing to do
                Self::shutdown_inner(&shutdown_state).ok();
            }
        });

        self
    }

    #[must_use]
    pub fn messages_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, Message> + 'static + Send,
    {
        self.messages_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn edited_messages_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, Message> + 'static + Send,
    {
        self.edited_messages_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn channel_posts_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, Message> + 'static + Send,
    {
        self.channel_posts_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn edited_channel_posts_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, Message> + 'static + Send,
    {
        self.edited_channel_posts_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn inline_queries_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, InlineQuery> + 'static + Send,
    {
        self.inline_queries_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn chosen_inline_results_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, ChosenInlineResult> + 'static + Send,
    {
        self.chosen_inline_results_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn callback_queries_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, CallbackQuery> + 'static + Send,
    {
        self.callback_queries_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn shipping_queries_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, ShippingQuery> + 'static + Send,
    {
        self.shipping_queries_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn pre_checkout_queries_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, PreCheckoutQuery> + 'static + Send,
    {
        self.pre_checkout_queries_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn polls_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, Poll> + 'static + Send,
    {
        self.polls_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn poll_answers_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, PollAnswer> + 'static + Send,
    {
        self.poll_answers_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn my_chat_members_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, ChatMemberUpdated> + 'static + Send,
    {
        self.my_chat_members_queue = self.new_tx(h);
        self
    }

    #[must_use]
    pub fn chat_members_handler<H>(mut self, h: H) -> Self
    where
        H: DispatcherHandler<R, ChatMemberUpdated> + 'static + Send,
    {
        self.chat_members_queue = self.new_tx(h);
        self
    }

    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener).
    pub async fn dispatch(&self)
    where
        R: Requester + Clone,
        <R as Requester>::GetUpdatesFaultTolerant: Send,
    {
        self.dispatch_with_listener(
            update_listeners::polling_default(self.requester.clone()).await,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    pub async fn dispatch_with_listener<'a, UListener, ListenerE, Eh>(
        &'a self,
        mut update_listener: UListener,
        update_listener_error_handler: Arc<Eh>,
    ) where
        UListener: UpdateListener<ListenerE> + 'a,
        Eh: ErrorHandler<ListenerE> + 'a,
        ListenerE: Debug,
        R: Requester + Clone,
    {
        use ShutdownState::*;

        const MIN_SHUTDOWN_CHECK_TIMEOUT: Duration = Duration::from_secs(1);

        let shutdown_check_timeout =
            update_listener.timeout_hint().unwrap_or(Duration::ZERO) + MIN_SHUTDOWN_CHECK_TIMEOUT;

        if let Err(_) = self.shutdown_state.compare_exchange(IsntRunning, Running) {
            panic!("Dispatching is already running");
        }

        {
            let stream = update_listener.as_stream();
            tokio::pin!(stream);

            loop {
                if let Ok(upd) = timeout(shutdown_check_timeout, stream.next()).await {
                    match upd {
                        None => break,
                        Some(upd) => self.process_update(upd, &update_listener_error_handler).await,
                    }
                }

                if let ShuttingDown = self.shutdown_state.load() {
                    log::debug!("Start shutting down dispatching");
                    break;
                }
            }
        }

        update_listener.stop();

        update_listener
            .as_stream()
            .for_each(|upd| self.process_update(upd, &update_listener_error_handler))
            .await;

        if let ShuttingDown = self.shutdown_state.load() {
            // Stopped because of a `shutdown` call.

            // Notify `shutdown`s that we finished
            self.shutdown_notify_back.notify_waiters();
            log::debug!("Dispatching shut down");
        } else {
            log::debug!("Dispatching stopped (listner returned `None`)");
        }

        self.shutdown_state.store(IsntRunning);
    }

    /// Tries to shutdown dispatching.
    ///
    /// Returns error if this dispather isn't dispatching at the moment.
    ///
    /// If you don't need to wait for shutdown, returned future can be ignored.
    pub fn shutdown(&self) -> Result<impl Future<Output = ()> + '_, ShutdownError> {
        Self::shutdown_inner(&self.shutdown_state)
            .map(|()| async move { self.shutdown_notify_back.notified().await })
    }

    fn shutdown_inner(shutdown_state: &AtomicShutdownState) -> Result<(), ShutdownError> {
        use ShutdownState::*;

        let res = shutdown_state.compare_exchange(Running, ShuttingDown);

        match res {
            Ok(_) | Err(ShuttingDown) => Ok(()),
            Err(IsntRunning) => return Err(ShutdownError::IsntRunning),
            Err(Running) => unreachable!(),
        }
    }

    async fn process_update<ListenerE, Eh>(
        &self,
        update: Result<Update, ListenerE>,
        update_listener_error_handler: &Arc<Eh>,
    ) where
        R: Requester + Clone,
        Eh: ErrorHandler<ListenerE>,
        ListenerE: Debug,
    {
        {
            log::trace!("Dispatcher received an update: {:?}", update);

            let update = match update {
                Ok(update) => update,
                Err(error) => {
                    Arc::clone(update_listener_error_handler).handle_error(error).await;
                    return;
                }
            };

            match update.kind {
                UpdateKind::Message(message) => {
                    send!(&self.requester, &self.messages_queue, message, UpdateKind::Message);
                }
                UpdateKind::EditedMessage(message) => {
                    send!(
                        &self.requester,
                        &self.edited_messages_queue,
                        message,
                        UpdateKind::EditedMessage
                    );
                }
                UpdateKind::ChannelPost(post) => {
                    send!(
                        &self.requester,
                        &self.channel_posts_queue,
                        post,
                        UpdateKind::ChannelPost
                    );
                }
                UpdateKind::EditedChannelPost(post) => {
                    send!(
                        &self.requester,
                        &self.edited_channel_posts_queue,
                        post,
                        UpdateKind::EditedChannelPost
                    );
                }
                UpdateKind::InlineQuery(query) => {
                    send!(
                        &self.requester,
                        &self.inline_queries_queue,
                        query,
                        UpdateKind::InlineQuery
                    );
                }
                UpdateKind::ChosenInlineResult(result) => {
                    send!(
                        &self.requester,
                        &self.chosen_inline_results_queue,
                        result,
                        UpdateKind::ChosenInlineResult
                    );
                }
                UpdateKind::CallbackQuery(query) => {
                    send!(
                        &self.requester,
                        &self.callback_queries_queue,
                        query,
                        UpdateKind::CallbackQuer
                    );
                }
                UpdateKind::ShippingQuery(query) => {
                    send!(
                        &self.requester,
                        &self.shipping_queries_queue,
                        query,
                        UpdateKind::ShippingQuery
                    );
                }
                UpdateKind::PreCheckoutQuery(query) => {
                    send!(
                        &self.requester,
                        &self.pre_checkout_queries_queue,
                        query,
                        UpdateKind::PreCheckoutQuery
                    );
                }
                UpdateKind::Poll(poll) => {
                    send!(&self.requester, &self.polls_queue, poll, UpdateKind::Poll);
                }
                UpdateKind::PollAnswer(answer) => {
                    send!(
                        &self.requester,
                        &self.poll_answers_queue,
                        answer,
                        UpdateKind::PollAnswer
                    );
                }
                UpdateKind::MyChatMember(chat_member_updated) => {
                    send!(
                        &self.requester,
                        &self.my_chat_members_queue,
                        chat_member_updated,
                        UpdateKind::MyChatMember
                    );
                }
                UpdateKind::ChatMember(chat_member_updated) => {
                    send!(
                        &self.requester,
                        &self.chat_members_queue,
                        chat_member_updated,
                        UpdateKind::MyChatMember
                    );
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum ShutdownError {
    IsntRunning,
}

struct AtomicShutdownState {
    inner: AtomicU8,
}

impl AtomicShutdownState {
    fn load(&self) -> ShutdownState {
        ShutdownState::from_u8(self.inner.load(Ordering::SeqCst))
    }

    fn store(&self, new: ShutdownState) {
        self.inner.store(new as _, Ordering::SeqCst)
    }

    fn compare_exchange(
        &self,
        current: ShutdownState,
        new: ShutdownState,
    ) -> Result<ShutdownState, ShutdownState> {
        self.inner
            .compare_exchange(current as _, new as _, Ordering::SeqCst, Ordering::SeqCst)
            .map(ShutdownState::from_u8)
            .map_err(ShutdownState::from_u8)
    }
}

#[repr(u8)]
enum ShutdownState {
    Running,
    ShuttingDown,
    IsntRunning,
}

impl ShutdownState {
    fn from_u8(n: u8) -> Self {
        const RUNNING: u8 = ShutdownState::Running as u8;
        const SHUTTING_DOWN: u8 = ShutdownState::ShuttingDown as u8;
        const ISNT_RUNNING: u8 = ShutdownState::IsntRunning as u8;

        match n {
            RUNNING => ShutdownState::Running,
            SHUTTING_DOWN => ShutdownState::ShuttingDown,
            ISNT_RUNNING => ShutdownState::IsntRunning,
            _ => unreachable!(),
        }
    }
}
