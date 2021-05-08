pub mod serializer;

mod in_mem_storage;
mod trace_storage;

#[cfg(feature = "redis-storage")]
mod redis_storage;

#[cfg(feature = "sqlite-storage")]
mod sqlite_storage;

use futures::future::BoxFuture;

pub use self::{
    in_mem_storage::{InMemStorage, InMemStorageError},
    trace_storage::TraceStorage,
};

#[cfg(feature = "redis-storage")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "redis-storage")))]
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
/// `Storage` is used only to store dialogue states, i.e. it can't be used as a
/// generic database.
///
/// Currently we support the following storages out of the box:
///
/// - [`InMemStorage`] -- a storage based on [`std::collections::HashMap`].
/// - [`RedisStorage`] -- a Redis-based storage.
/// - [`SqliteStorage`] -- an SQLite-based persistent storage.
///
/// [`InMemStorage`]: crate::dispatching::dialogue::InMemStorage
/// [`RedisStorage`]: crate::dispatching::dialogue::RedisStorage
/// [`SqliteStorage`]: crate::dispatching::dialogue::SqliteStorage
pub trait Storage<D> {
    type Error;

    /// Removes a dialogue indexed by `chat_id`.
    ///
    /// If the dialogue indexed by `chat_id` does not exist, this function
    /// results in an error.
    #[must_use = "Futures are lazy and do nothing unless polled with .await"]
    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static;

    /// Updates a dialogue indexed by `chat_id` with `dialogue`.
    #[must_use = "Futures are lazy and do nothing unless polled with .await"]
    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static;

    /// Extracts a dialogue indexed by `chat_id`.
    #[must_use = "Futures are lazy and do nothing unless polled with .await"]
    fn get_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>;
}
