use crate::{
    dispatching::{
        dialogue::{DialogueDispatcher, DialogueStage, DialogueWithCx},
        update_listeners,
        update_listeners::UpdateListener,
        Dispatcher, UpdateWithCx,
    },
    error_handlers::{LoggingErrorHandler, OnError},
    types::Message,
    Bot,
};
use std::{convert::Infallible, fmt::Debug, future::Future};

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
pub async fn dialogues_repl<'a, H, Upd, D, Fut, HandlerE>(
    bot: Bot,
    bot_name: &'static str,
    handler: H,
) where
    H: Fn(UpdateWithCx<Upd>, D) -> Fut + Send + Sync + 'static,
    D: Default + Send + 'static,
    Fut: Future<Output = Result<DialogueStage<D>, HandlerE>> + Send + Sync + 'static,
    Result<DialogueStage<D>, HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
{
    let cloned_bot = bot.clone();

    dialogues_repl_with_listener(
        bot,
        bot_name,
        handler,
        update_listeners::polling_default(cloned_bot),
    )
    .await;
}

/// Like [`dialogue_repl`], but with a custom [`UpdateListener`].
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
/// [`dialogue_repl`]: crate::dispatching::repls::dialogue_repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
pub async fn dialogues_repl_with_listener<'a, H, D, Fut, L, ListenerE, HandlerE>(
    bot: Bot,
    bot_name: &'static str,
    handler: H,
    listener: L,
) where
    H: Fn(UpdateWithCx<Message>, D) -> Fut + Send + Sync + 'static,
    D: Default + Send + 'static,
    Fut: Future<Output = Result<DialogueStage<D>, HandlerE>> + Send + Sync + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    Result<DialogueStage<D>, HandlerE>: OnError<HandlerE>,
    HandlerE: Debug + Send,
{
    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |DialogueWithCx { cx, dialogue }: DialogueWithCx<Message, D, Infallible>| async move {
                let dialogue = dialogue.expect("std::convert::Infallible");
                handler(cx, dialogue).await.log_on_error().await
            },
        ))
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await
}
