use async_trait::async_trait;

use super::Storage;
use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct InMemStorage<Session> {
    map: HashMap<i64, Session>,
}

#[async_trait(?Send)]
#[async_trait]
impl<Session> Storage<Session> for InMemStorage<Session> {
    async fn remove_session(&mut self, chat_id: i64) -> Option<Session> {
        self.map.remove(&chat_id)
    }

    async fn update_session(
        &mut self,
        chat_id: i64,
        state: Session,
    ) -> Option<Session> {
        self.map.insert(chat_id, state)
    }
}
