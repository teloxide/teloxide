use async_trait::async_trait;

use super::Storage;
use crate::dispatching::dialogue::Dialogue;
use std::collections::HashMap;
use tokio::sync::Mutex;

/// A memory storage based on a hash map. Stores all the dialogues directly in
/// RAM.
///
/// ## Note
/// All the dialogues will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you need to implement a storage
/// communicating with a DB.
#[derive(Debug, Default)]
pub struct InMemStorage<State, T> {
    map: Mutex<HashMap<i64, Dialogue<State, T>>>,
}

#[async_trait(?Send)]
#[async_trait]
impl<State, T> Storage<State, T> for InMemStorage<State, T> {
    async fn remove_dialogue(
        &self,
        chat_id: i64,
    ) -> Option<Dialogue<State, T>> {
        self.map.lock().await.remove(&chat_id)
    }

    async fn update_dialogue(
        &self,
        chat_id: i64,
        dialogue: Dialogue<State, T>,
    ) -> Option<Dialogue<State, T>> {
        self.map.lock().await.insert(chat_id, dialogue)
    }
}
