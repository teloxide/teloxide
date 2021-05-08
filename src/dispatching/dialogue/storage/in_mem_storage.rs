use super::Storage;
use futures::future::BoxFuture;
use std::{collections::HashMap, sync::Arc};
use thiserror::Error;
use tokio::sync::Mutex;

/// An error returned from [`InMemStorage`].
#[derive(Debug, Error)]
pub enum InMemStorageError {
    /// Returned from [`InMemStorage::remove_dialogue`].
    #[error("row not found")]
    RowNotFound,
}

/// A dialogue storage based on [`std::collections::HashMap`].
///
/// ## Note
/// All your dialogues will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you should use e.g.
/// [`super::SqliteStorage`] or implement your own.
#[derive(Debug)]
pub struct InMemStorage<D> {
    map: Mutex<HashMap<i64, D>>,
}

impl<S> InMemStorage<S> {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self { map: Mutex::new(HashMap::new()) })
    }
}

impl<D> Storage<D> for InMemStorage<D>
where
    D: Clone,
    D: Send + 'static,
{
    type Error = InMemStorageError;

    fn remove_dialogue(self: Arc<Self>, chat_id: i64) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            self.map
                .lock()
                .await
                .remove(&chat_id)
                .map_or_else(|| Err(InMemStorageError::RowNotFound), |_| Ok(()))
        })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            self.map.lock().await.insert(chat_id, dialogue);
            Ok(())
        })
    }

    fn get_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>> {
        Box::pin(async move { Ok(self.map.lock().await.get(&chat_id).map(ToOwned::to_owned)) })
    }
}
