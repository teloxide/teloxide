use super::{serializer::Serializer, Storage};
use futures::future::BoxFuture;
use mongodb::{Client, options::ClientOptions};
use std::{
    convert::Infallible,
    fmt::{Debug, Display},
    sync::Arc,
};
use thiserror::Error;
use tokio::sync::Mutex;

/// An error returned from [`MongoStorage`].
///
/// [`MongoStorage`]: struct.MongoStorage.html
#[derive(Debug, Error)]
pub enum MongoStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("parsing/serializing error: {0}")]
    SerdeError(SE),
    #[error("error from MongoDB: {0}")]
    MongoError(#[from] mongodb::error::Error),
}

/// A memory storage based on [MongoDB](https://mongodb.com/).
pub struct MongoStorage<S> {
    conn: Mutex<mongodb::Client>,
    serializer: S,
}

impl<S> MongoStorage<S> {
    pub async fn open(
        url: &str,
        serializer: S,
    ) -> Result<Arc<Self>, MongoStorageError<Infallible>> {
        Ok(Arc::new(Self {
            conn: Mutex::new(Client::with_options(ClientOptions::parse(url).await?)?),
            serializer,
        }))
    }
}
