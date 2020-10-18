use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, Dispatcher, DispatcherHandlerRx,
        DispatcherHandlerRxExt, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    types::Message,
    utils::command::BotCommand,
    Bot,
};
use futures::StreamExt;
use std::{fmt::Debug, future::Future, sync::Arc};

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
pub async fn commands_repl<Cmd, H, Fut, HandlerE, N>(bot: Bot, bot_name: N, handler: H)
where
    Cmd: BotCommand + Send + 'static,
    H: Fn(UpdateWithCx<Message>, Cmd) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerE>> + Send + 'static,
    Result<(), HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
    N: Into<String> + Send + 'static,
{
    let cloned_bot = bot.clone();

    commands_repl_with_listener(
        bot,
        bot_name,
        handler,
        update_listeners::polling_default(cloned_bot),
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
pub async fn commands_repl_with_listener<'a, Cmd, H, Fut, L, ListenerE, HandlerE, N>(
    bot: Bot,
    bot_name: N,
    handler: H,
    listener: L,
) where
    Cmd: BotCommand + Send + 'static,
    H: Fn(UpdateWithCx<Message>, Cmd) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), HandlerE>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    Result<(), HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
    N: Into<String> + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(bot)
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.commands::<Cmd, N>(bot_name).for_each_concurrent(None, move |(cx, cmd)| {
                let handler = Arc::clone(&handler);

                async move {
                    handler(cx, cmd).await.log_on_error().await;
                }
            })
        })
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await
}
