use super::Storage;
use futures::future::BoxFuture;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

/// A memory storage based on a hash map. Stores all the dialogues directly in
/// RAM.
///
/// ## Note
/// All the dialogues will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you should use [`SqliteStorage`],
/// [`RedisStorage`] or implement your own.
///
/// [`RedisStorage`]: crate::dispatching::dialogue::RedisStorage
/// [`SqliteStorage`]: crate::dispatching::dialogue::SqliteStorage
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
    D: ToOwned<Owned = D>,
    D: Send + 'static,
{
    type Error = std::convert::Infallible;

    fn remove_dialogue(self: Arc<Self>, chat_id: i64) -> BoxFuture<'static, Result<(), Self::Error>>
    where
        D: Send + 'static,
    {
        Box::pin(async move {
            self.map.lock().await.remove(&chat_id);
            Ok(())
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
