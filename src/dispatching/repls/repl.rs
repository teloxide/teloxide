use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, DispatcherBuilder,
        UpdateWithCx,
    },
    types::Message,
    Bot,
};
use std::{fmt::Debug, future::Future, sync::Arc};
use crate::dispatching::error_handlers::{OnError, LoggingErrorHandler};
use crate::requests::Request;
use crate::dispatching::updates;

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
pub async fn repl<H, Fut, E>(bot: Bot, handler: H)
where
    H: Fn(UpdateWithCx<Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    Result<(), E>: OnError<E>,
    E: Debug + Send,
{
    let cloned_bot = bot.clone();
    repl_with_listener(bot, handler, update_listeners::polling_default(cloned_bot)).await;
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
pub async fn repl_with_listener<'a, H, Fut, E, L, ListenerE>(bot: Bot, handler: H, listener: L)
where
    H: Fn(UpdateWithCx<Message>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), E>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug,
    Result<(), E>: OnError<E>,
    E: Debug + Send,
{
    let handler = Arc::new(handler);

    let username = bot.get_me().send().await.unwrap().user.username.unwrap();

    DispatcherBuilder::new(
        bot,
        username,
    )
        .handle(
            updates::message()
                .by(
            move |cx: UpdateWithCx<Message>| {
            let handler = Arc::clone(&handler);

            async move {
                handler(cx).await.log_on_error().await;
            }

        }))
        .error_handler(LoggingErrorHandler::with_custom_text("An error from the handler"))
        .build()
        .dispatch_with_listener(
            listener,
            &LoggingErrorHandler::with_custom_text("An error from the update listener")
        )
        .await;
}
