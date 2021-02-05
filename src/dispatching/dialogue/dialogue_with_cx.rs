use crate::{
    dispatching::{
        core::{FromContextOwn, GetCtx},
        dialogue::{dialogue_ctx::DialogueContext, GetChatId, Storage},
        UpdateWithCx,
    },
};
use crate::dispatching::dialogue::Dialogue;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub struct DialogueWithCx<Upd, D, E> {
    pub cx: UpdateWithCx<Upd>,
    pub dialogue: Option<Dialogue<D, E>>,
}

impl<Upd, D, S, E, Ctx> FromContextOwn<Ctx, DialogueContext<Upd, D, S>>
    for DialogueWithCx<Upd, D, E>
where
    Ctx: GetCtx<DialogueContext<Upd, D, S>>,
    S: Storage<D, Error = E> + Send + Sync + 'static,
{
    fn from_context(context: Ctx) -> Self {
        let cx = context.get_own();
        let DialogueContext { dispatcher_ctx, storage, dialogue, senders, chat_id } = cx;
        DialogueWithCx {
            cx: UpdateWithCx::from_context(dispatcher_ctx),
            dialogue: {
                match (dialogue, chat_id) {
                    (Some(d), Some(chat_id)) => {
                        Some(Dialogue { data: d, storage, senders, chat_id })
                    }
                    _ => None,
                }
            },
        }
    }
}

impl<Upd, D, E> GetChatId for DialogueWithCx<Upd, D, E>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.cx.update.chat_id()
    }
}
