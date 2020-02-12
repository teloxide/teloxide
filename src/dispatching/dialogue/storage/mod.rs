mod in_mem_storage;

use async_trait::async_trait;
pub use in_mem_storage::InMemStorage;

/// A storage of dialogues.
///
/// You can implement this trait for a structure that communicates with a DB and
/// be sure that after you restart your bot, all the dialogues won't be lost.
///
/// For a storage based on a simple hash map, see [`InMemStorage`].
///
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
#[async_trait(?Send)]
#[async_trait]
pub trait Storage<D> {
    /// Removes a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was deleted.
    async fn remove_dialogue(&self, chat_id: i64) -> Option<D>;

    /// Updates a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was updated.
    async fn update_dialogue(&self, chat_id: i64, dialogue: D) -> Option<D>;
}
