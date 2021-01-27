use crate::{
    dispatching::{
        core::HandleResult,
        dialogue::{DialogueDispatcherBuilder, DialogueWithCx, InMemStorage},
        error_handlers::LoggingErrorHandler,
        update_listeners,
        update_listeners::UpdateListener,
        updates,
    },
    requests::Request,
    types::Message,
    Bot,
};
use std::{convert::Infallible, fmt::Debug, future::Future, sync::Arc};

/// A [REPL] for dialogues.
///
/// All errors from an update listener and handler will be logged. This function
/// uses [`InMemStorage`].
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
pub async fn dialogues_repl<'a, H, D, E, Fut>(bot: Bot, handler: H)
where
    H: Fn(DialogueWithCx<Message, D, Infallible>) -> Fut + Send + Sync + 'static,
    D: Default + Debug + Clone + Send + Sync + 'static,
    Fut: Future + Send + 'static,
    Fut::Output: Into<HandleResult<E>> + Send + 'static,
    E: Debug + Send + 'static,
{
    let cloned_bot = bot.clone();

    dialogues_repl_with_listener(bot, handler, update_listeners::polling_default(cloned_bot)).await;
}

/// Like [`dialogues_repl`], but with a custom [`UpdateListener`].
///
/// All errors from an update listener and handler will be logged. This function
/// uses [`InMemStorage`].
///
/// # Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`dialogues_repl`]: crate::dispatching::repls::dialogues_repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
pub async fn dialogues_repl_with_listener<'a, H, D, Fut, L, E, ListenerE>(
    bot: Bot,
    handler: H,
    listener: L,
) where
    H: Fn(DialogueWithCx<Message, D, Infallible>) -> Fut + Send + Sync + 'static,
    D: Default + Debug + Clone + Send + Sync + 'static,
    Fut: Future + Send + 'static,
    Fut::Output: Into<HandleResult<E>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    E: Debug + Send + 'static,
    ListenerE: Debug + Send + 'a,
{
    let handler = Arc::new(handler);

    let username = bot.get_me().send().await.unwrap().user.username.unwrap();

    DialogueDispatcherBuilder::new(bot, username, InMemStorage::new())
        .handle(updates::message().by(move |cx: DialogueWithCx<Message, D, Infallible>| {
            let handler = Arc::clone(&handler);
            handler(cx)
        }))
        .error_handler(LoggingErrorHandler::with_custom_text("An error from the handler"))
        .build()
        .dispatch_with_listener(
            listener,
            &LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
