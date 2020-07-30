use crate::{
    dispatching::{Dispatcher, DispatcherHandlerRx, UpdateWithCx},
    error_handlers::OnError,
    types::Message,
    Bot,
};
use futures::StreamExt;
use std::{fmt::Debug, future::Future, sync::Arc};

/// A [REPL] for messages.
///
/// All errors from an update listener will be logged.
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub async fn repl<H, Fut, E>(bot: Bot, handler: H)
where
    H: Fn(UpdateWithCx<Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    Result<(), E>: OnError<E>,
    E: Debug + Send,
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
