use crate::{
    dispatching::{
        stop_token::StopToken, update_listeners, update_listeners::UpdateListener, ShutdownToken,
    },
    error_handlers::{ErrorHandler, LoggingErrorHandler},
    requests::Requester,
    types::{AllowedUpdate, Update},
    utils::shutdown_token::shutdown_check_timeout_for,
};

use dptree::di::{DependencyMap, DependencySupplier};
use futures::{future::BoxFuture, StreamExt};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    ops::{ControlFlow, Deref},
    sync::Arc,
};
use teloxide_core::{requests::Request, types::UpdateKind};
use tokio::time::timeout;
use tokio_stream::wrappers::ReceiverStream;

use std::future::Future;

/// The builder for [`Dispatcher`].
pub struct DispatcherBuilder<R, Err> {
    bot: R,
    dependencies: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: Arc<DefaultHandler>,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
}

impl<R, Err> DispatcherBuilder<R, Err>
where
    R: Clone + Requester + Clone + Send + Sync + 'static,
    Err: Debug + Send + Sync + 'static,
{
    /// Specifies a handler that will be called for an unhandled update.
    ///
    /// By default, it is a mere [`log::warn`].
    #[must_use]
    pub fn default_handler<H, Fut>(self, handler: H) -> Self
    where
        H: Fn(Arc<Update>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = ()> + Send + 'static,
    {
        let handler = Arc::new(handler);

        Self {
            default_handler: Arc::new(Box::new(move |upd| {
                let handler = Arc::clone(&handler);
                Box::pin(handler(upd))
            })),
            ..self
        }
    }

    /// Specifies a handler that will be called on a handler error.
    ///
    /// By default, it is [`LoggingErrorHandler`].
    #[must_use]
    pub fn error_handler(self, handler: Arc<dyn ErrorHandler<Err> + Send + Sync>) -> Self {
        Self { error_handler: handler, ..self }
    }

    /// Specifies dependencies that can be used inside of handlers.
    ///
    /// By default, there is no dependencies.
    #[must_use]
    pub fn dependencies(self, dependencies: DependencyMap) -> Self {
        Self { dependencies, ..self }
    }

    /// Constructs [`Dispatcher`].
    #[must_use]
    pub fn build(self) -> Dispatcher<R, Err> {
        Dispatcher {
            bot: self.bot.clone(),
            dependencies: self.dependencies,
            handler: Arc::clone(&self.handler),
            default_handler: Arc::clone(&self.default_handler),
            error_handler: Arc::clone(&self.error_handler),
            allowed_updates: Default::default(),
            state: ShutdownToken::new(),
            workers: HashMap::new(),
            default_worker: None,
        }
    }
}

/// The base for update dispatching.
///
/// Updates from different chats are handles concurrently, whereas updates from
/// the same chats are handled sequentially. If the dispatcher is unable to
/// determine a chat ID of an incoming update, it will be handled concurrently.
pub struct Dispatcher<R, Err> {
    bot: R,
    dependencies: DependencyMap,

    handler: Arc<UpdateHandler<Err>>,
    default_handler: Arc<DefaultHandler>,
    workers: HashMap<i64, WorkerTx>,
    default_worker: Option<WorkerTx>,

    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
    // TODO: respect allowed_udpates
    allowed_updates: HashSet<AllowedUpdate>,

    state: ShutdownToken,
}

type WorkerTx = tokio::sync::mpsc::Sender<Update>;

// TODO: it is allowed to return message as response on telegram request in
// webhooks, so we can allow this too. See more there: https://core.telegram.org/bots/api#making-requests-when-getting-updates

/// A handler that processes updates from Telegram.
pub type UpdateHandler<Err> = dptree::Handler<'static, DependencyMap, Result<(), Err>>;

type DefaultHandler = Box<dyn Fn(Arc<Update>) -> BoxFuture<'static, ()> + Send + Sync>;

impl<R, Err> Dispatcher<R, Err>
where
    R: Requester + Clone + Send + Sync + 'static,
    Err: Send + Sync + 'static,
{
    /// Constructs a new [`DispatcherBuilder`] with `bot` and `handler`.
    #[must_use]
    pub fn builder(bot: R, handler: UpdateHandler<Err>) -> DispatcherBuilder<R, Err>
    where
        Err: Debug,
    {
        DispatcherBuilder {
            bot,
            dependencies: DependencyMap::new(),
            handler: Arc::new(handler),
            default_handler: Arc::new(Box::new(|upd| {
                log::warn!("Unhandled update: {:?}", upd);
                Box::pin(async {})
            })),
            error_handler: LoggingErrorHandler::new(),
        }
    }

    /// Starts your bot with the default parameters.
    ///
    /// The default parameters are a long polling update listener and log all
    /// errors produced by this listener.
    ///
    /// Each time a handler is invoked, [`Dispatcher`] adds the following
    /// dependencies (in addition to those passed to
    /// [`DispatcherBuilder::dependencies`]):
    ///
    ///  - Your bot passed to [`Dispatcher::builder`];
    ///  - An update from Telegram;
    ///  - [`crate::types::Me`] (can be used in [`HandlerExt::filter_command`]).
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    /// [a ctrlc signal]: Dispatcher::setup_ctrlc_handler
    /// [`HandlerExt::filter_command`]: crate::dispatching::HandlerExt::filter_command
    pub async fn dispatch(&mut self)
    where
        R: Requester + Clone,
        <R as Requester>::GetUpdates: Send,
    {
        let listener = update_listeners::polling_default(self.bot.clone()).await;
        let error_handler =
            LoggingErrorHandler::with_custom_text("An error from the update listener");

        self.dispatch_with_listener(listener, error_handler).await;
    }

    /// Starts your bot with custom `update_listener` and
    /// `update_listener_error_handler`.
    ///
    /// This method adds the same dependencies as [`Dispatcher::dispatch`].
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
    {
        // FIXME: there should be a way to check if dependency is already inserted
        let me = self.bot.get_me().send().await.expect("Failed to retrieve 'me'");
        self.dependencies.insert(me);
        self.dependencies.insert(self.bot.clone());

        update_listener.hint_allowed_updates(&mut self.allowed_updates.clone().into_iter());

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

        self.workers.drain();
        self.default_worker.take();
        self.state.done();
    }

    async fn process_update<LErr, LErrHandler>(
        &mut self,
        update: Result<Update, LErr>,
        err_handler: &Arc<LErrHandler>,
    ) where
        LErrHandler: ErrorHandler<LErr>,
    {
        match update {
            Ok(upd) => {
                if let UpdateKind::Error(err) = upd.kind {
                    log::error!(
                        "Cannot parse an update.\nError: {:?}\n\
                            This is a bug in teloxide-core, please open an issue here: \
                            https://github.com/teloxide/teloxide/issues.",
                        err,
                    );
                    return;
                }

                let deps = self.dependencies.clone();
                let handler = Arc::clone(&self.handler);
                let default_handler = Arc::clone(&self.default_handler);
                let error_handler = Arc::clone(&self.error_handler);

                let tx = match upd.chat() {
                    Some(chat) => self.workers.entry(chat.id).or_insert_with(|| {
                        spawn_worker(deps, handler, default_handler, error_handler)
                    }),
                    None => self.default_worker.get_or_insert_with(|| {
                        spawn_default_worker(deps, handler, default_handler, error_handler)
                    }),
                };

                tx.send(upd).await.expect("TX is dead");
            }
            Err(err) => err_handler.clone().handle_error(err).await,
        }
    }

    /// Setups the `^C` handler that [`shutdown`]s dispatching.
    ///
    /// [`shutdown`]: ShutdownToken::shutdown
    #[cfg(feature = "ctrlc_handler")]
    pub fn setup_ctrlc_handler(&mut self) -> &mut Self {
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

    /// Returns a shutdown token, which can later be used to shutdown
    /// dispatching.
    pub fn shutdown_token(&self) -> ShutdownToken {
        self.state.clone()
    }
}

const WORKER_QUEUE_SIZE: usize = 64;

fn spawn_worker<Err>(
    deps: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: Arc<DefaultHandler>,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
) -> WorkerTx
where
    Err: Send + Sync + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::channel(WORKER_QUEUE_SIZE);

    let deps = Arc::new(deps);

    tokio::spawn(async move {
        ReceiverStream::new(rx)
            .for_each(|update| {
                let deps = Arc::clone(&deps);
                let handler = Arc::clone(&handler);
                let default_handler = Arc::clone(&default_handler);
                let error_handler = Arc::clone(&error_handler);

                async move {
                    handle_update(update, deps, handler, default_handler, error_handler).await;
                }
            })
            .await;
    });

    tx
}

fn spawn_default_worker<Err>(
    deps: DependencyMap,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: Arc<DefaultHandler>,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
) -> WorkerTx
where
    Err: Send + Sync + 'static,
{
    let (tx, rx) = tokio::sync::mpsc::channel(WORKER_QUEUE_SIZE);

    let deps = Arc::new(deps);

    tokio::spawn(async move {
        ReceiverStream::new(rx)
            .for_each_concurrent(None, |update| {
                let deps = Arc::clone(&deps);
                let handler = Arc::clone(&handler);
                let default_handler = Arc::clone(&default_handler);
                let error_handler = Arc::clone(&error_handler);

                async move {
                    handle_update(update, deps, handler, default_handler, error_handler).await;
                }
            })
            .await;
    });

    tx
}

async fn handle_update<Err>(
    update: Update,
    deps: Arc<DependencyMap>,
    handler: Arc<UpdateHandler<Err>>,
    default_handler: Arc<DefaultHandler>,
    error_handler: Arc<dyn ErrorHandler<Err> + Send + Sync>,
) where
    Err: Send + Sync + 'static,
{
    let mut deps = deps.deref().clone();
    deps.insert(update);

    match handler.dispatch(deps).await {
        ControlFlow::Break(Ok(())) => {}
        ControlFlow::Break(Err(err)) => error_handler.clone().handle_error(err).await,
        ControlFlow::Continue(deps) => {
            let update = deps.get();
            (default_handler)(update).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use teloxide_core::Bot;

    use super::*;

    #[tokio::test]
    async fn test_tokio_spawn() {
        tokio::spawn(async {
            // Just check that this code compiles.
            if false {
                Dispatcher::<_, Infallible>::builder(Bot::new(""), dptree::entry())
                    .build()
                    .dispatch()
                    .await;
            }
        })
        .await
        .unwrap();
    }
}
