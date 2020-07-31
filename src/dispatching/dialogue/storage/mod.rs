pub mod serializer;

mod in_mem_storage;

#[cfg(feature = "redis-storage")]
mod redis_storage;

#[cfg(feature = "sqlite-storage")]
mod sqlite_storage;

use futures::future::BoxFuture;

pub use in_mem_storage::InMemStorage;
#[cfg(feature = "redis-storage")]
pub use redis_storage::{RedisStorage, RedisStorageError};
pub use serializer::Serializer;
use std::sync::Arc;

#[cfg(feature = "sqlite-storage")]
pub use sqlite_storage::{SqliteStorage, SqliteStorageError};

/// A storage of dialogues.
///
/// You can implement this trait for a structure that communicates with a DB and
/// be sure that after you restart your bot, all the dialogues won't be lost.
///
/// For a storage based on a simple hash map, see [`InMemStorage`].
///
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
pub trait Storage<D> {
    type Error;

    /// Removes a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was deleted.
    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static;

    /// Updates a dialogue with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a dialogue, `Some(dialogue)` if a
    /// `dialogue` was updated.
    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static;
}
