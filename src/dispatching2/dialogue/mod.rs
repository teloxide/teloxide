#[cfg(feature = "redis-storage")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "redis-storage")))]
pub use storage::{RedisStorage, RedisStorageError};

#[cfg(feature = "sqlite-storage")]
pub use storage::{SqliteStorage, SqliteStorageError};

pub use storage::{serializer, InMemStorage, InMemStorageError, Serializer, Storage, TraceStorage};

pub use dialogue_handler_ext::DialogueHandlerExt;

use std::{future::Future, marker::PhantomData, sync::Arc};

mod dialogue_handler_ext;
mod get_chat_id;
mod storage;

#[derive(Debug)]
pub struct Dialogue<D, S> {
    // Maybe it's better to use Box<dyn Storage<D, Err>> here but it's require
    // us to introduce `Err` generic parameter.
    storage: Arc<S>,
    chat_id: i64,
    _phantom: PhantomData<D>,
}

// #[derive] requires generics to implement Clone,
// but `S` wrapped around Arc, and `D` wrapped around PhantomData.
impl<D, S> Clone for Dialogue<D, S> {
    fn clone(&self) -> Self {
        Dialogue {
            storage: self.storage.clone(),
            chat_id: self.chat_id.clone(),
            _phantom: PhantomData,
        }
    }
}

impl<D, S> Dialogue<D, S>
where
    D: Send + 'static,
    S: Storage<D>,
{
    pub fn new(storage: Arc<S>, chat_id: i64) -> Result<Self, S::Error> {
        Ok(Self { storage, chat_id, _phantom: PhantomData })
    }

    // TODO: Cache this.
    pub async fn current_state(&self) -> Result<Option<D>, S::Error> {
        self.storage.clone().get_dialogue(self.chat_id).await
    }

    pub async fn current_state_or_default(&self) -> Result<D, S::Error>
    where
        D: Default,
    {
        match self.storage.clone().get_dialogue(self.chat_id).await? {
            Some(d) => Ok(d),
            None => {
                self.storage.clone().update_dialogue(self.chat_id, D::default()).await?;
                Ok(D::default())
            }
        }
    }

    pub async fn next<State>(&self, state: State) -> Result<(), S::Error>
    where
        D: From<State>,
    {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    pub async fn with<F, Fut, State>(&self, f: F) -> Result<(), S::Error>
    where
        F: FnOnce(Option<D>) -> Fut,
        Fut: Future<Output = State>,
        D: From<State>,
    {
        let current_dialogue = self.current_state().await?;
        let new_dialogue = f(current_dialogue).await.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    pub async fn reset(&self) -> Result<(), S::Error>
    where
        D: Default,
    {
        self.next(D::default()).await
    }

    pub async fn exit(&self) -> Result<(), S::Error> {
        self.storage.clone().remove_dialogue(self.chat_id).await
    }
}
