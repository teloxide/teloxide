use super::{serializer::Serializer, Storage};
use deadpool_redis::{redis, CreatePoolError, PoolError, Runtime};
use futures::future::BoxFuture;
use redis::AsyncCommands;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use teloxide_core::types::ChatId;
use thiserror::Error;

/// An error returned from [`RedisStorage`].
#[derive(Debug, Error)]
pub enum RedisStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("parsing/serializing error: {0}")]
    SerdeError(SE),

    #[error("error from Redis: {0}")]
    RedisError(#[from] redis::RedisError),

    #[error("error creating redis pool: {0}")]
    CreatePoolError(#[from] CreatePoolError),

    #[error("redis pool error: {0}")]
    PoolError(#[from] PoolError),

    /// Returned from [`RedisStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

/// A dialogue storage based on [Redis](https://redis.io/).
pub struct RedisStorage<S> {
    pool: deadpool_redis::Pool,
    serializer: S,
}

impl<S> RedisStorage<S> {
    pub async fn open(
        url: &str,
        serializer: S,
    ) -> Result<Arc<Self>, RedisStorageError<Infallible>> {
        let config = deadpool_redis::Config::from_url(url);
        let pool = config.create_pool(Some(Runtime::Tokio1))?;

        Ok(Arc::new(Self { pool, serializer }))
    }
}

impl<S, D> Storage<D> for RedisStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = RedisStorageError<<S as Serializer<D>>::Error>;

    fn remove_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let mut conn = self.pool.get().await?;

            let deleted_rows_count =
                redis::pipe().atomic().del(chat_id).query_async(&mut conn).await?;

            if let redis::Value::Array(values) = deleted_rows_count {
                // False positive
                #[allow(clippy::collapsible_match)]
                if let redis::Value::Int(deleted_rows_count) = values[0] {
                    match deleted_rows_count {
                        0 => return Err(RedisStorageError::DialogueNotFound),
                        _ => return Ok(()),
                    }
                }
            }

            unreachable!("Must return redis::Value::Bulk(redis::Value::Int(_))");
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let dialogue =
                self.serializer.serialize(&dialogue).map_err(RedisStorageError::SerdeError)?;
            () = self.pool.get().await?.set::<_, Vec<u8>, _>(chat_id, dialogue).await?;
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        ChatId(chat_id): ChatId,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            self.pool
                .get()
                .await?
                .get::<_, Option<Vec<u8>>>(chat_id)
                .await?
                .map(|d| self.serializer.deserialize(&d).map_err(RedisStorageError::SerdeError))
                .transpose()
        })
    }
}
