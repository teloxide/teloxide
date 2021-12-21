use crate::dispatching2::dialogue::{get_chat_id::GetChatId, Dialogue, Storage};
use dptree::{di::DependencyMap, Handler, Insert};
use std::sync::Arc;

pub trait DialogueHandlerExt {
    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        D: Send + Sync + 'static,
        Upd: GetChatId + Send + Sync + 'static;
}

impl<'a, Output> DialogueHandlerExt for Handler<'a, DependencyMap, Output>
where
    Output: Send + Sync + 'static,
{
    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        // FIXME: some of this requirements are useless.
        S: Storage<D> + Send + Sync + 'static,
        D: Send + Sync + 'static,
        Upd: GetChatId + Send + Sync + 'static,
    {
        self.chain(dptree::filter_map(|storage: Arc<S>, upd: Arc<Upd>| async move {
            let chat_id = upd.chat_id()?;
            Dialogue::new(storage, chat_id).ok()
        }))
    }
}
