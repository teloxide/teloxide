use crate::{
    dispatching::{Dispatcher, DispatcherHandlerRx, DispatcherHandlerRxExt, UpdateWithCx},
    error_handlers::OnError,
    requests::ResponseResult,
    types::Message,
    utils::command::BotCommand,
    Bot,
};
use futures::StreamExt;
use std::{future::Future, sync::Arc};

/// A [REPL] for commands.
///
/// Used mostly for testing and demonstrative purposes.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and [`repl`],
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`repl`]: crate::dispatching::repl
pub async fn commands_repl<Cmd, H, Fut>(bot: Bot, bot_name: &'static str, handler: H)
where
    Cmd: BotCommand + Send + 'static,
    H: Fn(UpdateWithCx<Message>, Cmd) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ResponseResult<()>> + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(bot)
        .messages_handler(move |rx: DispatcherHandlerRx<Message>| {
            rx.commands::<Cmd, &'static str>(bot_name).for_each_concurrent(
                None,
                move |(cx, cmd)| {
                    let handler = Arc::clone(&handler);

                    async move {
                        handler(cx, cmd).await.log_on_error().await;
                    }
                },
            )
        })
        .dispatch()
        .await;
}
