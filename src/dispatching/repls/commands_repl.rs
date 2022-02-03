use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, Dispatcher, DispatcherHandlerRx,
        DispatcherHandlerRxExt, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    utils::command::BotCommand,
};
use futures::StreamExt;
use std::{fmt::Debug, future::Future, sync::Arc};
use teloxide_core::{requests::Requester, types::Message};
use tokio_stream::wrappers::UnboundedReceiverStream;

/// A [REPL] for commands.
///
/// All errors from an update listener and handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub async fn commands_repl<R, Cmd, H, Fut, HandlerE, N>(requester: R, bot_name: N, handler: H)
where
    Cmd: BotCommand + Send + 'static,
    H: Fn(UpdateWithCx<R, Message>, Cmd) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerE>> + Send + 'static,
    Result<(), HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
    N: Into<String> + Send + 'static,
    R: Requester + Send + Clone + 'static,
    <R as Requester>::GetUpdates: Send,
{
    let cloned_requester = requester.clone();

    commands_repl_with_listener(
        requester,
        bot_name,
        handler,
        update_listeners::polling_default(cloned_requester).await,
    )
    .await;
}

/// Like [`commands_repl`], but with a custom [`UpdateListener`].
///
/// All errors from an update listener and handler will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`commands_repl`]: crate::dispatching::repls::commands_repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
pub async fn commands_repl_with_listener<'a, R, Cmd, H, Fut, L, ListenerE, HandlerE, N>(
    requester: R,
    bot_name: N,
    handler: H,
    listener: L,
) where
    Cmd: BotCommand + Send + 'static,
    H: Fn(UpdateWithCx<R, Message>, Cmd) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerE>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    Result<(), HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
    N: Into<String> + Send + 'static,
    R: Requester + Clone + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::<R>::new(requester)
        .messages_handler(move |rx: DispatcherHandlerRx<R, Message>| {
            UnboundedReceiverStream::new(rx).commands::<Cmd, N>(bot_name).for_each_concurrent(
                None,
                move |(cx, cmd)| {
                    let handler = Arc::clone(&handler);

                    async move {
                        handler(cx, cmd).await.log_on_error().await;
                    }
                },
            )
        })
        .setup_ctrlc_handler()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await
}
