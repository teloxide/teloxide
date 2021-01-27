use crate::dispatching::{dialogue::GetChatId, UpdateWithCx};
use crate::dispatching::core::{GetCtx, FromContextOwn};
use crate::dispatching::dialogue::dialogue_ctx::DialogueContext;
use std::sync::Arc;
use crate::dispatching::dialogue::{Storage, DialogueStage};
use lockfree::map::Map;
use tokio::sync::mpsc;
use crate::types::Update;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub struct DialogueWithCx<Upd, D, E> {
    pub cx: UpdateWithCx<Upd>,
    pub dialogue: Option<D>,
    storage: Arc<dyn Storage<D, Error = E> + Send + Sync>,
    senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
}

impl<Upd, D, S, E, Ctx> FromContextOwn<Ctx, DialogueContext<Upd, D, S>> for DialogueWithCx<Upd, D, E>
where
    Ctx: GetCtx<DialogueContext<Upd, D, S>>,
    S: Storage<D, Error = E> + Send + Sync + 'static,
{
    fn from_context(context: Ctx) -> Self {
        let cx = context.get_own();
        let DialogueContext { dispatcher_ctx, storage, dialogue, senders, chat_id: _ } = cx;
        DialogueWithCx {
            cx: UpdateWithCx::from_context(dispatcher_ctx),
            storage,
            dialogue,
            senders,
        }
    }
}

impl<Upd, D, E> DialogueWithCx<Upd, D, E>
where
    Upd: GetChatId,
    D: Default + Send + 'static,
{
    pub async fn next(self, factory: impl Fn(D) -> DialogueStage<D>) -> UpdateWithCx<Upd> {
        let chat_id = self.cx.update.chat_id();
        let DialogueWithCx { cx, dialogue, storage, senders } = self;
        let next = factory(dialogue.unwrap_or_default());

        match next {
            DialogueStage::Next(new_dialogue) => {
                if let Ok(Some(_)) = storage.update_dialogue(chat_id, new_dialogue).await {
                    panic!(
                        "Oops, you have an bug in your Storage: update_dialogue returns \
                         Some after remove_dialogue"
                    );
                }
            }
            DialogueStage::Exit => {
                // On the next .poll() call, the spawned future will
                // return Poll::Ready, because we are dropping the
                // sender right here:
                senders.remove(&chat_id);

                // We already removed a dialogue from `storage` (see
                // the beginning of this async block).
            }
        }

        cx
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
