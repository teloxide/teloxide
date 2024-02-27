pub mod serializer;

mod in_mem_storage;
mod trace_storage;

#[cfg(feature = "redis-storage")]
mod redis_storage;

#[cfg(any(feature = "sqlite-storage-nativetls", feature = "sqlite-storage-rustls"))]
mod sqlite_storage;

#[cfg(feature = "postgres-storage-nativetls")]
mod postgres_storage;

use futures::future::BoxFuture;
use teloxide_core::types::ChatId;

pub use self::{
    in_mem_storage::{InMemStorage, InMemStorageError},
    trace_storage::TraceStorage,
};

#[cfg(feature = "redis-storage")]
pub use redis_storage::{RedisStorage, RedisStorageError};
pub use serializer::Serializer;
use std::sync::Arc;

#[cfg(any(feature = "sqlite-storage-nativetls", feature = "sqlite-storage-rustls"))]
pub use sqlite_storage::{SqliteStorage, SqliteStorageError};

#[cfg(feature = "postgres-storage-nativetls")]
pub use postgres_storage::{PostgresStorage, PostgresStorageError};

/// A storage with an erased error type.
pub type ErasedStorage<D> =
    dyn Storage<D, Error = Box<dyn std::error::Error + Send + Sync>> + Send + Sync;

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
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static;

    /// Updates a dialogue indexed by `chat_id` with `dialogue`.
    #[must_use = "Futures are lazy and do nothing unless polled with .await"]
    fn update_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static;

    /// Returns the dialogue indexed by `chat_id`.
    #[must_use = "Futures are lazy and do nothing unless polled with .await"]
    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>;

    /// Erases [`Self::Error`] to [`std::error::Error`].
    #[must_use]
    fn erase(self: Arc<Self>) -> Arc<ErasedStorage<D>>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Error: std::error::Error + Send + Sync + 'static,
    {
        Arc::new(Eraser(self))
    }
}

struct Eraser<S>(Arc<S>);

impl<D, S> Storage<D> for Eraser<S>
where
    S: Storage<D> + Send + Sync + 'static,
    S::Error: std::error::Error + Send + Sync + 'static,
{
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(
            async move { Arc::clone(&self.0).remove_dialogue(chat_id).await.map_err(|e| e.into()) },
        )
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            Arc::clone(&self.0).update_dialogue(chat_id, dialogue).await.map_err(|e| e.into())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(
            async move { Arc::clone(&self.0).get_dialogue(chat_id).await.map_err(|e| e.into()) },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_erased() {
        let chat_id = ChatId(123);

        let erased = InMemStorage::new().erase();
        Arc::clone(&erased).update_dialogue(chat_id, 1).await.unwrap();
        assert_eq!(Arc::clone(&erased).get_dialogue(chat_id).await.unwrap().unwrap(), 1);
        Arc::clone(&erased).remove_dialogue(chat_id).await.unwrap();
        assert_eq!(Arc::clone(&erased).get_dialogue(chat_id).await.unwrap(), None);
    }
}
