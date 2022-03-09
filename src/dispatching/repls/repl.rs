use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, Dispatcher, DispatcherHandlerRx,
        UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
};
use futures::StreamExt;
use std::{fmt::Debug, future::Future, sync::Arc};
use teloxide_core::{requests::Requester, types::Message};
use tokio_stream::wrappers::UnboundedReceiverStream;

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
pub async fn repl<R, H, Fut, E>(requester: R, handler: H)
where
    H: Fn(UpdateWithCx<R, Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    Result<(), E>: OnError<E>,
    E: Debug + Send,
    R: Requester + Send + Clone + 'static,
    <R as Requester>::GetUpdates: Send,
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
pub async fn repl_with_listener<'a, R, H, Fut, E, L, ListenerE>(
    requester: R,
    handler: H,
    listener: L,
) where
    H: Fn(UpdateWithCx<R, Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug,
    Result<(), E>: OnError<E>,
    E: Debug + Send,
    R: Requester + Clone + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(requester)
        .messages_handler(|rx: DispatcherHandlerRx<R, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, move |message| {
                let handler = Arc::clone(&handler);

                async move {
                    handler(message).await.log_on_error().await;
                }
            })
        })
        .setup_ctrlc_handler()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}

#[test]
fn repl_is_send() {
    let bot = crate::Bot::new("");
    let repl = crate::repl(bot, |_| async { crate::respond(()) });
    assert_send(&repl);

    fn assert_send(_: &impl Send) {}
}
