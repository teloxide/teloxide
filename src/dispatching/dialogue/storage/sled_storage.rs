use bytes::Bytes;
use futures::future::BoxFuture;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::{Debug, Display};
use std::sync::Arc;
use teloxide::dispatching::dialogue::{Serializer, Storage};
use teloxide::prelude::*;
use thiserror::Error;

#[derive(Debug)]
pub struct SledStorage<S> {
    tree: sled::Tree,
    serializer: S,
}

/// An error returned from [`SledStorageError`].
#[derive(Debug, Error)]
pub enum SledStorageError<SE>
where
    SE: Debug + Display,
{
    #[error("dialogue serialization error: {0}")]
    SerdeError(SE),

    #[error("sled error: {0}")]
    SledError(#[from] sled::Error),

    /// Returned from [`SqliteStorage::remove_dialogue`].
    #[error("row not found")]
    DialogueNotFound,
}

impl<S> SledStorage<S> {
    pub fn with_tree(tree: sled::Tree, serializer: S) -> color_eyre::Result<Arc<Self>> {
        Ok(Arc::new(Self { tree, serializer }))
    }

    pub fn with_db(db: sled::Db, serializer: S) -> color_eyre::Result<Arc<Self>> {
        Self::with_tree(db.open_tree("dialogue")?, serializer)
    }

    pub fn with_config(config: sled::Config, serializer: S) -> color_eyre::Result<Arc<Self>> {
        Self::with_db(config.open()?, serializer)
    }
}

#[async_trait::async_trait]
impl<S, D> Storage<D> for SledStorage<S>
where
    S: Send + Sync + Serializer<D> + 'static,
    D: Send + Serialize + DeserializeOwned + 'static,
    <S as Serializer<D>>::Error: Debug + Display,
{
    type Error = SledStorageError<<S as Serializer<D>>::Error>;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, color_eyre::Result<(), Self::Error>> {
        Box::pin(async move {
            if self
                .tree
                .remove(chat_id.0.to_le_bytes())
                .map_err(SledStorageError::SledError)?
                .is_none()
            {
                Err(SledStorageError::DialogueNotFound)
            } else {
                Ok(())
            }
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
        dialogue: D,
    ) -> BoxFuture<'static, color_eyre::Result<(), Self::Error>> {
        Box::pin(async move {
            let dialogue: Bytes = self
                .serializer
                .serialize(&dialogue)
                .map_err(SledStorageError::SerdeError)?
                .into();
            self.tree
                .update_and_fetch(chat_id.0.to_le_bytes(), |_| Some(dialogue.as_ref()))?;
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: ChatId,
    ) -> BoxFuture<'static, color_eyre::Result<Option<D>, Self::Error>> {
        Box::pin(async move {
            self.tree
                .get(chat_id.0.to_le_bytes())?
                .map(|d| {
                    self.serializer
                        .deserialize(&d)
                        .map_err(SledStorageError::SerdeError)
                })
                .transpose()
        })
    }
}
