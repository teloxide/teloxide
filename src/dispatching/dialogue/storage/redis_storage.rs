use super::{serializer::Serializer, Storage};
use futures::future::BoxFuture;
use redis::{AsyncCommands, FromRedisValue, IntoConnectionInfo};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    ops::DerefMut,
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

/// An error returned from [`RedisStorage`].
///
/// [`RedisStorage`]: struct.RedisStorage.html
#[derive(Debug, Error)]
pub enum RedisStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("parsing/serializing error: {0}")]
    SerdeError(SE),
    #[error("error from Redis: {0}")]
    RedisError(#[from] redis::RedisError),
}

/// A memory storage based on [Redis](https://redis.io/).
pub struct RedisStorage<S> {
    conn: Mutex<redis::aio::Connection>,
    serializer: S,
}

impl<S> RedisStorage<S> {
    pub async fn open(
        url: impl IntoConnectionInfo,
        serializer: S,
    ) -> Result<Arc<Self>, RedisStorageError<Infallible>> {
        Ok(Arc::new(Self {
            conn: Mutex::new(redis::Client::open(url)?.get_async_connection().await?),
            serializer,
        }))
    }
}

impl<S, D> Storage<D> for RedisStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = RedisStorageError<<S as Serializer<D>>::Error>;

    // `.del().ignore()` is much more readable than `.del()\n.ignore()`
    #[rustfmt::skip]
    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let res = redis::pipe()
                .atomic()
                .get(chat_id)
                .del(chat_id).ignore()
                .query_async::<_, redis::Value>(
                    self.conn.lock().await.deref_mut(),
                )
                .await?;
            // We're expecting `.pipe()` to return us an exactly one result in
            // bulk, so all other branches should be unreachable
            match res {
                redis::Value::Bulk(bulk) if bulk.len() == 1 => {
                    Option::<Vec<u8>>::from_redis_value(&bulk[0])?
                        .map(|v| {
                            self.serializer
                                .deserialize(&v)
                                .map_err(RedisStorageError::SerdeError)
                        })
                        .transpose()?;
                    Ok(())
                }
                _ => unreachable!(),
            }
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>> {
        Box::pin(async move {
            let dialogue =
                self.serializer.serialize(&dialogue).map_err(RedisStorageError::SerdeError)?;
            self.conn.lock().await.set::<_, Vec<u8>, _>(chat_id, dialogue).await?;
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            self.conn
                .lock()
                .await
                .get::<_, Option<Vec<u8>>>(chat_id)
                .await?
                .map(|d| self.serializer.deserialize(&d).map_err(RedisStorageError::SerdeError))
                .transpose()
        })
    }
}
