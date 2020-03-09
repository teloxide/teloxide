use super::Storage;
use futures::future::BoxFuture;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// A memory storage based on a hash map. Stores all the dialogues directly in
/// RAM.
///
/// ## Note
/// All the dialogues will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you need to implement a storage
/// communicating with a DB.
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

impl<D> Storage<D> for InMemStorage<D> {
    type Error = std::convert::Infallible;

    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move { Ok(self.map.lock().await.remove(&chat_id)) })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> BoxFuture<'static, Result<Option<D>, Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(
            async move { Ok(self.map.lock().await.insert(chat_id, dialogue)) },
        )
    }
}
