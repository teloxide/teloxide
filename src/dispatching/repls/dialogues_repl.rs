use crate::{
    dispatching::{
        dialogue::{DialogueDispatcher, DialogueStage, DialogueWithCx, InMemStorageError},
        update_listeners,
        update_listeners::UpdateListener,
        Dispatcher, UpdateWithCx,
    },
    error_handlers::LoggingErrorHandler,
};
use std::{fmt::Debug, future::Future, sync::Arc};
use teloxide_core::{requests::Requester, types::Message};

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
#[cfg(feature = "ctrlc_handler")]
pub async fn dialogues_repl<'a, R, H, D, Fut>(requester: R, handler: H)
where
    H: Fn(UpdateWithCx<R, Message>, D) -> Fut + Send + Sync + 'static,
    D: Clone + Default + Send + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
    R: Requester + Send + Clone + 'static,
    <R as Requester>::GetUpdatesFaultTolerant: Send,
{
    let cloned_requester = requester.clone();

    dialogues_repl_with_listener(
        requester,
        handler,
        update_listeners::polling_default(cloned_requester).await,
    )
    .await;
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
#[cfg(feature = "ctrlc_handler")]
pub async fn dialogues_repl_with_listener<'a, R, H, D, Fut, L, ListenerE>(
    requester: R,
    handler: H,
    listener: L,
) where
    H: Fn(UpdateWithCx<R, Message>, D) -> Fut + Send + Sync + 'static,
    D: Clone + Default + Send + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    R: Requester + Send + Clone + 'static,
{
    let handler = Arc::new(handler);

    Dispatcher::new(requester)
        .messages_handler(DialogueDispatcher::new(
            move |DialogueWithCx { cx, dialogue }: DialogueWithCx<
                R,
                Message,
                D,
                InMemStorageError,
            >| {
                let handler = Arc::clone(&handler);

                async move {
                    let dialogue = dialogue.expect("std::convert::Infallible");
                    handler(cx, dialogue).await
                }
            },
        ))
        .setup_ctrlc_handler()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
