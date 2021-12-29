use crate::dispatching2::dialogue::{get_chat_id::GetChatId, Dialogue, Storage};
use dptree::{di::DependencyMap, Handler};
use std::sync::Arc;

pub trait DialogueHandlerExt {
    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static;
}

impl<'a, Output> DialogueHandlerExt for Handler<'a, DependencyMap, Output>
where
    Output: Send + Sync + 'static,
{
    fn add_dialogue<Upd, S, D>(self) -> Self
    where
        // FIXME: some of this requirements are useless.
        S: Storage<D> + Send + Sync + 'static,
        <S as Storage<D>>::Error: Send,
        D: Default + Send + Sync + 'static,
        Upd: GetChatId + Clone + Send + Sync + 'static,
    {
        self.chain(
            dptree::filter_map(|storage: Arc<S>, upd: Upd| async move {
                let chat_id = upd.chat_id()?;
                Dialogue::new(storage, chat_id).ok()
            })
            .chain(dptree::filter_map(|dialogue: Dialogue<D, S>| async move {
                dialogue.current_state_or_default().await.ok()
            })),
        )
    }
}
