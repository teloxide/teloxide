use crate::{
    dispatching::{
        core::{FromContextOwn, GetCtx},
        dialogue::{dialogue_ctx::DialogueContext, GetChatId, Storage},
        UpdateWithCx,
    },
    types::Update,
};
use lockfree::map::Map;
use std::sync::Arc;
use tokio::sync::mpsc;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub struct DialogueWithCx<Upd, D, E> {
    pub cx: UpdateWithCx<Upd>,
    pub dialogue: Dialogue<D, E>
}

pub struct Dialogue<D, E> {
    pub data: Option<D>,
    pub chat_id: Option<i64>,
    storage: Arc<dyn Storage<D, Error = E> + Send + Sync>,
    senders: Arc<Map<i64, mpsc::UnboundedSender<Update>>>,
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
            dialogue: Dialogue {
                data: dialogue,
                storage,
                senders,
                chat_id,
            }
        }
    }
}

impl<D, E> Dialogue<D, E>
where
    D: Default + Send + 'static,
{
    pub async fn next(self, factory: impl Fn(D) -> D) -> Result<(), ()> {
        let Dialogue { data, chat_id, storage, .. } = self;
        let chat_id = chat_id.ok_or(())?;
        let next = factory(data.unwrap_or_default());

        if let Ok(Some(_)) = storage.update_dialogue(chat_id, next).await {
            panic!(
                "Oops, you have an bug in your Storage: update_dialogue returns Some \
                 after remove_dialogue"
            );
        }

        Ok(())
    }

    pub async fn exit(self) -> Result<(), ()> {
        let Dialogue { chat_id, senders, .. } = self;
        let chat_id = chat_id.ok_or(())?;

        // On the next .poll() call, the spawned future will
        // return Poll::Ready, because we are dropping the
        // sender right here:
        senders.remove(&chat_id);

        // We already removed a dialogue from `storage` (see
        // the beginning of this async block).

        Ok(())
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
