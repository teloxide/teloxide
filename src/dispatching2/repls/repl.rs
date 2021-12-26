use crate::{
    dispatching::{update_listeners, update_listeners::UpdateListener},
    dispatching2::Dispatcher,
    error_handlers::{LoggingErrorHandler, OnError},
};
use dptree::di::{DependencyMap, Injectable};
use std::{fmt::Debug, sync::Arc};
use teloxide_core::requests::Requester;

/// A [REPL] for messages.
///
/// All errors from an update listener and a handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[cfg(feature = "ctrlc_handler")]
pub async fn repl<R, H, E, Args>(requester: R, handler: H)
where
    H: Injectable<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    Result<(), E>: OnError<E>,
    E: Debug + Send + Sync + 'static,
    R: Requester + Send + Sync + Clone + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    let cloned_requester = requester.clone();
    repl_with_listener(
        requester,
        handler,
        update_listeners::polling_default(cloned_requester).await,
    )
    .await;
}

/// Like [`repl`], but with a custom [`UpdateListener`].
///
/// All errors from an update listener and handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`repl`]: crate::dispatching::repls::repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
#[cfg(feature = "ctrlc_handler")]
pub async fn repl_with_listener<'a, R, H, E, L, ListenerE, Args>(
    requester: R,
    handler: H,
    listener: L,
) where
    H: Injectable<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug,
    Result<(), E>: OnError<E>,
    E: Debug + Send + Sync + 'static,
    R: Requester + Send + Sync + 'static,
{
    #[allow(unused_mut)]
    let mut dispatcher = Dispatcher::new(Arc::new(requester))
        .messages_handler(|h| h.branch(dptree::endpoint(handler)));

    #[cfg(feature = "ctrlc_handler")]
    let mut dispatcher = dispatcher.setup_ctrlc_handler();

    dispatcher
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
