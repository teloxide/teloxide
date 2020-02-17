mod in_mem_storage;

pub use in_mem_storage::InMemStorage;
use std::{future::Future, pin::Pin, sync::Arc};

/// A storage of dialogues.
///
/// You can implement this trait for a structure that communicates with a DB and
/// be sure that after you restart your bot, all the dialogues won't be lost.
///
/// For a storage based on a simple hash map, see [`InMemStorage`].
///
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
pub trait Storage<D> {
    /// Removes a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was deleted.
    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> Pin<Box<dyn Future<Output = Option<D>> + Send + Sync + 'static>>
    where
        D: Send + Sync + 'static;

    /// Updates a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was updated.
    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> Pin<Box<dyn Future<Output = Option<D>> + Send + Sync + 'static>>
    where
        D: Send + Sync + 'static;
}
