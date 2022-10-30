use super::{serializer::Serializer, Storage};
use futures::future::BoxFuture;
use rocksdb::{DBCompressionType, DBWithThreadMode, MultiThreaded};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    str,
    sync::Arc,
};
use teloxide_core::types::ChatId;
use thiserror::Error;

/// A persistent dialogue storage based on [RocksDb](http://rocksdb.org/).
pub struct RocksDbStorage<S> {
    db: DBWithThreadMode<MultiThreaded>,
    serializer: S,
}

/// An error returned from [`RocksDbStorage`].
#[derive(Debug, Error)]
pub enum RocksDbStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),

    #[error("RocksDb error: {0}")]
    RocksDbError(#[from] rocksdb::Error),

    /// Returned from [`RocksDbStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

impl<S> RocksDbStorage<S> {
    pub async fn open(
        path: &str,
        serializer: S,
        options: Option<rocksdb::Options>,
    ) -> Result<Arc<Self>, RocksDbStorageError<Infallible>> {
        let options = match options {
            Some(opts) => opts,
            None => {
                let mut opts = rocksdb::Options::default();
                opts.set_compression_type(DBCompressionType::Lz4);
                opts.create_if_missing(true);
                opts
            }
        };

        let db = DBWithThreadMode::<MultiThreaded>::open(&options, path)?;
        Ok(Arc::new(Self { db, serializer }))
    }
}

impl<S, D> Storage<D> for RocksDbStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = RocksDbStorageError<<S as Serializer<D>>::Error>;

    /// Returns [`RocksDbStorageError::DialogueNotFound`] if a dialogue does not
    /// exist.
    fn remove_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let key = chat_id.to_le_bytes();

            if self.db.get(&key)?.is_none() {
                return Err(RocksDbStorageError::DialogueNotFound);
            }

            self.db.delete(&key).unwrap();

            Ok(())
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let d =
                self.serializer.serialize(&dialogue).map_err(RocksDbStorageError::SerdeError)?;

            let key = chat_id.to_le_bytes();
            self.db.put(&key, &d)?;

            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            let key = chat_id.to_le_bytes();
            self.db
                .get(&key)?
                .map(|d| self.serializer.deserialize(&d).map_err(RocksDbStorageError::SerdeError))
                .transpose()
        })
    }
}
