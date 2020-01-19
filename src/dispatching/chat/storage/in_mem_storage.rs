use async_trait::async_trait;

use super::Storage;
use std::collections::HashMap;

/// A memory storage based on a hash map. Stores all the sessions directly in
/// RAM.
///
/// ## Note
/// All the sessions will be lost after you restart your bot. If you need to
/// store them somewhere on a drive, you need to implement a storage
/// communicating with a DB.
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
