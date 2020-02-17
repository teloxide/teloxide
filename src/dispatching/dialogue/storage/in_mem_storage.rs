use super::Storage;
use std::{collections::HashMap, future::Future, pin::Pin, sync::Arc};
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
        Arc::new(Self {
            map: Mutex::new(HashMap::new()),
        })
    }
}

impl<D> Storage<D> for InMemStorage<D> {
    fn remove_dialogue(
        self: Arc<Self>,
        chat_id: i64,
    ) -> Pin<Box<dyn Future<Output = Option<D>> + Send + Sync + 'static>>
    where
        D: Send + Sync + 'static,
    {
        Box::pin(async move { self.map.lock().await.remove(&chat_id) })
    }

    fn update_dialogue(
        self: Arc<Self>,
        chat_id: i64,
        dialogue: D,
    ) -> Pin<Box<dyn Future<Output = Option<D>> + Send + Sync + 'static>>
    where
        D: Send + Sync + 'static,
    {
        Box::pin(async move { self.map.lock().await.insert(chat_id, dialogue) })
    }
}
