mod in_mem_storage;

use async_trait::async_trait;
pub use in_mem_storage::InMemStorage;

#[async_trait(?Send)]
#[async_trait]
pub trait Storage<Session> {
    async fn remove_session(&mut self, chat_id: i64) -> Option<Session>;
    async fn update_session(
        &mut self,
        chat_id: i64,
        state: Session,
    ) -> Option<Session>;
}
