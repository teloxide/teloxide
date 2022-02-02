//! Support for user dialogues.

#[cfg(feature = "redis-storage")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "redis-storage")))]
pub use storage::{RedisStorage, RedisStorageError};

#[cfg(feature = "sqlite-storage")]
pub use storage::{SqliteStorage, SqliteStorageError};

pub use storage::{serializer, InMemStorage, InMemStorageError, Serializer, Storage, TraceStorage};

pub use dialogue_handler_ext::DialogueHandlerExt;

use std::{marker::PhantomData, sync::Arc};

mod dialogue_handler_ext;
mod get_chat_id;
mod storage;

/// A handle for controlling dialogue state.
#[derive(Debug)]
pub struct Dialogue<D, S> {
    storage: Arc<S>,
    chat_id: i64,
    _phantom: PhantomData<D>,
}

// `#[derive]` requires generics to implement `Clone`, but `S` is wrapped around
// `Arc`, and `D` is wrapped around PhantomData.
impl<D, S> Clone for Dialogue<D, S> {
    fn clone(&self) -> Self {
        Dialogue { storage: self.storage.clone(), chat_id: self.chat_id, _phantom: PhantomData }
    }
}

impl<D, S> Dialogue<D, S>
where
    D: Send + 'static,
    S: Storage<D>,
{
    /// Constructs a new dialogue with `storage` (where dialogues are stored)
    /// and `chat_id` of a current dialogue.
    pub fn new(storage: Arc<S>, chat_id: i64) -> Self {
        Self { storage, chat_id, _phantom: PhantomData }
    }

    /// Retrieves the current state of the dialogue or `None` if there is no
    /// dialogue.
    pub async fn get(&self) -> Result<Option<D>, S::Error> {
        self.storage.clone().get_dialogue(self.chat_id).await
    }

    /// Like [`Dialogue::get`] but returns a default value if there is no
    /// dialogue.
    pub async fn get_or_default(&self) -> Result<D, S::Error>
    where
        D: Default,
    {
        match self.get().await? {
            Some(d) => Ok(d),
            None => {
                self.storage.clone().update_dialogue(self.chat_id, D::default()).await?;
                Ok(D::default())
            }
        }
    }

    /// Updates the dialogue state.
    ///
    /// The dialogue type `D` must implement `From<State>` to allow implicit
    /// conversion from `State` to `D`.
    pub async fn update<State>(&self, state: State) -> Result<(), S::Error>
    where
        D: From<State>,
    {
        let new_dialogue = state.into();
        self.storage.clone().update_dialogue(self.chat_id, new_dialogue).await?;
        Ok(())
    }

    /// Updates the dialogue with a default value.
    pub async fn reset(&self) -> Result<(), S::Error>
    where
        D: Default,
    {
        self.update(D::default()).await
    }

    /// Removes the dialogue from the storage provided to [`Dialogue::new`].
    pub async fn exit(&self) -> Result<(), S::Error> {
        self.storage.clone().remove_dialogue(self.chat_id).await
    }
}
