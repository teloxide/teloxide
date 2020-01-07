use async_trait::async_trait;

use crate::dispatching::storage::Storage;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct InMemStorage<S> {
    map: HashMap<i64, S>,
}

#[async_trait(?Send)]
#[async_trait]
impl<S> Storage for InMemStorage<S> {
    type Session = S;

    async fn remove_session(&mut self, chat_id: i64) -> Option<S> {
        self.map.remove(&chat_id)
    }

    async fn update_session(&mut self, chat_id: i64, state: S) -> Option<S> {
        self.map.insert(chat_id, state)
    }
}
