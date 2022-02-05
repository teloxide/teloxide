use std::{fmt::Debug, sync::Arc};

use crate::{
    dispatching::{
        stop_token::StopToken,
        update_listeners::{self, UpdateListener},
        DispatcherHandler, UpdateWithCx,
    },
    error_handlers::{ErrorHandler, LoggingErrorHandler},
    utils::shutdown_token::shutdown_check_timeout_for,
};

use futures::{stream::FuturesUnordered, StreamExt};
use teloxide_core::{
    requests::Requester,
    types::{
        AllowedUpdate, CallbackQuery, ChatJoinRequest, ChatMemberUpdated, ChosenInlineResult,
        InlineQuery, Message, Poll, PollAnswer, PreCheckoutQuery, ShippingQuery, Update,
        UpdateKind,
    },
};
use tokio::{sync::mpsc, task::JoinHandle, time::timeout};

use crate::utils::shutdown_token::ShutdownToken;

type Tx<Upd, R> = Option<mpsc::UnboundedSender<UpdateWithCx<Upd, R>>>;

/// One dispatcher to rule them all.
///
/// See the [module-level documentation](crate::dispatching) for the design
/// overview.
#[deprecated(note = "Use dispatching2 instead")]
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
    chat_join_requests_queue: Tx<R, ChatJoinRequest>,

    running_handlers: FuturesUnordered<JoinHandle<()>>,

    state: ShutdownToken,
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
            chat_join_requests_queue: None,
            running_handlers: FuturesUnordered::new(),
            state: ShutdownToken::new(),
        }
    }

    #[must_use]
    fn new_tx<H, Upd>(&mut self, h: H) -> Tx<R, Upd>
    where
        H: DispatcherHandler<R, Upd> + Send + 'static,
        Upd: Send + 'static,
        R: Send + 'static,
    {
        let (tx, rx) = mpsc::unbounded_channel();
        let join_handle = tokio::spawn(h.handle(rx));

        self.running_handlers.push(join_handle);

        Some(tx)
    }

    /// Setup the `^C` handler which [`shutdown`]s dispatching.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    #[cfg(feature = "ctrlc_handler")]
    #[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "ctrlc_handler")))]
    #[must_use]
    pub fn setup_ctrlc_handler(self) -> Self {
        let token = self.state.clone();
        tokio::spawn(async move {
            loop {
                tokio::signal::ctrl_c().await.expect("Failed to listen for ^C");

                match token.shutdown() {
                    Ok(f) => {
                        log::info!("^C received, trying to shutdown the dispatcher...");
                        f.await;
                        log::info!("dispatcher is shutdown...");
                    }
                    Err(_) => {
                        log::info!("^C received, the dispatcher isn't running, ignoring the signal")
                    }
                }
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
    ///
    /// Please note that after shutting down (either because of [`shutdown`],
    /// [a ctrlc signal], or [`UpdateListener`] returning `None`) all handlers
    /// will be gone. As such, to restart listening you need to re-add
    /// handlers.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    /// [a ctrlc signal]: Dispatcher::setup_ctrlc_handler
    pub async fn dispatch(&mut self)
    where
        R: Requester + Clone,
        <R as Requester>::GetUpdates: Send,
    {
        let listener = update_listeners::polling_default(self.requester.clone()).await;
        let error_handler =
            LoggingErrorHandler::with_custom_text("An error from the update listener");

        self.dispatch_with_listener(listener, error_handler).await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    ///
    /// Please note that after shutting down (either because of [`shutdown`],
    /// [a ctrlc signal], or [`UpdateListener`] returning `None`) all handlers
    /// will be gone. As such, to restart listening you need to re-add
    /// handlers.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    /// [a ctrlc signal]: Dispatcher::setup_ctrlc_handler
    pub async fn dispatch_with_listener<'a, UListener, ListenerE, Eh>(
        &'a mut self,
        mut update_listener: UListener,
        update_listener_error_handler: Arc<Eh>,
    ) where
        UListener: UpdateListener<ListenerE> + 'a,
        Eh: ErrorHandler<ListenerE> + 'a,
        ListenerE: Debug,
        R: Requester + Clone,
    {
        self.hint_allowed_updates(&mut update_listener);

        let shutdown_check_timeout = shutdown_check_timeout_for(&update_listener);
        let mut stop_token = Some(update_listener.stop_token());

        self.state.start_dispatching();

        {
            let stream = update_listener.as_stream();
            tokio::pin!(stream);

            loop {
                // False positive
                #[allow(clippy::collapsible_match)]
                if let Ok(upd) = timeout(shutdown_check_timeout, stream.next()).await {
                    match upd {
                        None => break,
                        Some(upd) => self.process_update(upd, &update_listener_error_handler).await,
                    }
                }

                if self.state.is_shutting_down() {
                    if let Some(token) = stop_token.take() {
                        log::debug!("Start shutting down dispatching...");
                        token.stop();
                    }
                }
            }
        }

        self.wait_for_handlers().await;
        self.state.done();
    }

    /// Returns a shutdown token, which can later be used to shutdown
    /// dispatching.
    pub fn shutdown_token(&self) -> ShutdownToken {
        self.state.clone()
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
                    send(&self.requester, &self.messages_queue, message, "UpdateKind::Message")
                }
                UpdateKind::EditedMessage(message) => send(
                    &self.requester,
                    &self.edited_messages_queue,
                    message,
                    "UpdateKind::EditedMessage",
                ),
                UpdateKind::ChannelPost(post) => send(
                    &self.requester,
                    &self.channel_posts_queue,
                    post,
                    "UpdateKind::ChannelPost",
                ),
                UpdateKind::EditedChannelPost(post) => send(
                    &self.requester,
                    &self.edited_channel_posts_queue,
                    post,
                    "UpdateKind::EditedChannelPost",
                ),
                UpdateKind::InlineQuery(query) => send(
                    &self.requester,
                    &self.inline_queries_queue,
                    query,
                    "UpdateKind::InlineQuery",
                ),
                UpdateKind::ChosenInlineResult(result) => send(
                    &self.requester,
                    &self.chosen_inline_results_queue,
                    result,
                    "UpdateKind::ChosenInlineResult",
                ),
                UpdateKind::CallbackQuery(query) => send(
                    &self.requester,
                    &self.callback_queries_queue,
                    query,
                    "UpdateKind::CallbackQuer",
                ),
                UpdateKind::ShippingQuery(query) => send(
                    &self.requester,
                    &self.shipping_queries_queue,
                    query,
                    "UpdateKind::ShippingQuery",
                ),
                UpdateKind::PreCheckoutQuery(query) => send(
                    &self.requester,
                    &self.pre_checkout_queries_queue,
                    query,
                    "UpdateKind::PreCheckoutQuery",
                ),
                UpdateKind::Poll(poll) => {
                    send(&self.requester, &self.polls_queue, poll, "UpdateKind::Poll")
                }
                UpdateKind::PollAnswer(answer) => send(
                    &self.requester,
                    &self.poll_answers_queue,
                    answer,
                    "UpdateKind::PollAnswer",
                ),
                UpdateKind::MyChatMember(chat_member_updated) => send(
                    &self.requester,
                    &self.my_chat_members_queue,
                    chat_member_updated,
                    "UpdateKind::MyChatMember",
                ),
                UpdateKind::ChatMember(chat_member_updated) => send(
                    &self.requester,
                    &self.chat_members_queue,
                    chat_member_updated,
                    "UpdateKind::MyChatMember",
                ),
                UpdateKind::ChatJoinRequest(chat_join_request) => send(
                    &self.requester,
                    &self.chat_join_requests_queue,
                    chat_join_request,
                    "UpdateKind::ChatJoinRequest",
                ),
                UpdateKind::Error(err) => {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\n\
                            This is a bug in teloxide-core, please open an issue here: \
                            https://github.com/teloxide/teloxide-core/issues.",
                        err,
                    );
                }
            }
        }
    }

    fn hint_allowed_updates<E>(&self, listener: &mut impl UpdateListener<E>) {
        fn hint_handler_allowed_update<T>(
            queue: &Option<T>,
            kind: AllowedUpdate,
        ) -> std::option::IntoIter<AllowedUpdate> {
            queue.as_ref().map(|_| kind).into_iter()
        }

        let mut allowed = hint_handler_allowed_update(&self.messages_queue, AllowedUpdate::Message)
            .chain(hint_handler_allowed_update(
                &self.edited_messages_queue,
                AllowedUpdate::EditedMessage,
            ))
            .chain(hint_handler_allowed_update(
                &self.channel_posts_queue,
                AllowedUpdate::ChannelPost,
            ))
            .chain(hint_handler_allowed_update(
                &self.edited_channel_posts_queue,
                AllowedUpdate::EditedChannelPost,
            ))
            .chain(hint_handler_allowed_update(
                &self.inline_queries_queue,
                AllowedUpdate::InlineQuery,
            ))
            .chain(hint_handler_allowed_update(
                &self.chosen_inline_results_queue,
                AllowedUpdate::ChosenInlineResult,
            ))
            .chain(hint_handler_allowed_update(
                &self.callback_queries_queue,
                AllowedUpdate::CallbackQuery,
            ))
            .chain(hint_handler_allowed_update(
                &self.shipping_queries_queue,
                AllowedUpdate::ShippingQuery,
            ))
            .chain(hint_handler_allowed_update(
                &self.pre_checkout_queries_queue,
                AllowedUpdate::PreCheckoutQuery,
            ))
            .chain(hint_handler_allowed_update(&self.polls_queue, AllowedUpdate::Poll))
            .chain(hint_handler_allowed_update(&self.poll_answers_queue, AllowedUpdate::PollAnswer))
            .chain(hint_handler_allowed_update(
                &self.my_chat_members_queue,
                AllowedUpdate::MyChatMember,
            ))
            .chain(hint_handler_allowed_update(
                &self.chat_members_queue,
                AllowedUpdate::ChatMember,
            ));

        listener.hint_allowed_updates(&mut allowed);
    }

    async fn wait_for_handlers(&mut self) {
        log::debug!("Waiting for handlers to finish");

        // Drop all senders, so handlers can stop
        self.messages_queue.take();
        self.edited_messages_queue.take();
        self.channel_posts_queue.take();
        self.edited_channel_posts_queue.take();
        self.inline_queries_queue.take();
        self.chosen_inline_results_queue.take();
        self.callback_queries_queue.take();
        self.shipping_queries_queue.take();
        self.pre_checkout_queries_queue.take();
        self.polls_queue.take();
        self.poll_answers_queue.take();
        self.my_chat_members_queue.take();
        self.chat_members_queue.take();

        // Wait untill all handlers finish
        self.running_handlers.by_ref().for_each(|_| async {}).await;
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
