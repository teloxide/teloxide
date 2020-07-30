use crate::{
    dispatching::{Dispatcher, DispatcherHandlerRx, UpdateWithCx},
    error_handlers::OnError,
    requests::ResponseResult,
    types::Message,
    Bot,
};
use futures::StreamExt;
use std::{future::Future, sync::Arc};

/// A [REPL] for messages.
///
/// Used mostly for testing and demonstrative purposes.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`], because Telegram
/// disallow multiple requests at the same time from the same bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub async fn repl<H, Fut>(bot: Bot, handler: H)
where
    H: Fn(UpdateWithCx<Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ResponseResult<()>> + Send + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each_concurrent(None, move |message| {
                let handler = Arc::clone(&handler);

                async move {
                    handler(message).await.log_on_error().await;
                }
            })
        })
        .dispatch()
        .await;
}
