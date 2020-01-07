mod in_mem_storage;

use async_trait::async_trait;
pub use in_mem_storage::InMemStorage;

#[async_trait(?Send)]
#[async_trait]
pub trait Storage {
    type Session;

    async fn remove_session(&mut self, chat_id: i64) -> Option<Self::Session>;
    async fn update_session(
        &mut self,
        chat_id: i64,
        state: Self::Session,
    ) -> Option<Self::Session>;
}
