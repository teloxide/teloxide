use crate::dispatching::{
    dialogue::{
        Dialogue, DialogueHandlerCtx, DialogueStage, GetChatId, InMemStorage,
        Storage,
    },
    CtxHandler, DispatcherHandlerCtx,
};
use std::{future::Future, pin::Pin};

/// A dispatcher of dialogues.
///
/// Note that `DialogueDispatcher` implements `CtxHandler`, so you can just put
/// an instance of this dispatcher into the [`Dispatcher`]'s methods.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DialogueDispatcher<'a, State, T, H> {
    storage: Box<dyn Storage<State, T> + 'a>,
    handler: H,
}

impl<'a, State, T, H> DialogueDispatcher<'a, State, T, H>
where
    Dialogue<State, T>: Default + 'a,
    T: Default + 'a,
    State: Default + 'a,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
    #[must_use]
    pub fn new(handler: H) -> Self {
        Self {
            storage: Box::new(InMemStorage::default()),
            handler,
        }
    }

    /// Creates a dispatcher with the specified `handler` and `storage`.
    #[must_use]
    pub fn with_storage<Stg>(handler: H, storage: Stg) -> Self
    where
        Stg: Storage<State, T> + 'a,
    {
        Self {
            storage: Box::new(storage),
            handler,
        }
    }
}

impl<'a, State, T, H, Upd> CtxHandler<DispatcherHandlerCtx<Upd>, Result<(), ()>>
    for DialogueDispatcher<'a, State, T, H>
where
    H: CtxHandler<DialogueHandlerCtx<Upd, State, T>, DialogueStage<State, T>>,
    Upd: GetChatId,
    Dialogue<State, T>: Default,
{
    fn handle_ctx<'b>(
        &'b self,
        ctx: DispatcherHandlerCtx<Upd>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + 'b>>
    where
        Upd: 'b,
    {
        Box::pin(async move {
            let chat_id = ctx.update.chat_id();

            let dialogue = self
                .storage
                .remove_dialogue(chat_id)
                .await
                .unwrap_or_default();

            if let DialogueStage::Next(new_dialogue) = self
                .handler
                .handle_ctx(DialogueHandlerCtx {
                    bot: ctx.bot,
                    update: ctx.update,
                    dialogue,
                })
                .await
            {
                if self
                    .storage
                    .update_dialogue(chat_id, new_dialogue)
                    .await
                    .is_some()
                {
                    panic!(
                        "We previously storage.remove_dialogue() so \
                         storage.update_dialogue() must return None"
                    );
                }
            }

            Ok(())
        })
    }
}
