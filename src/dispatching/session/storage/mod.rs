mod in_mem_storage;

use async_trait::async_trait;
pub use in_mem_storage::InMemStorage;

/// A storage of sessions.
///
/// You can implement this trait for a structure that communicates with a DB and
/// be sure that after you restart your bot, all the sessions won't be lost.
///
/// For a storage based on a simple hash map, see [`InMemStorage`].
///
/// [`InMemStorage`]: crate::dispatching::private::InMemStorage
#[async_trait(?Send)]
#[async_trait]
pub trait Storage<Session> {
    /// Removes a session with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a session, `Some(session)` if a
    /// `session` was deleted.
    async fn remove_session(&mut self, chat_id: i64) -> Option<Session>;

    /// Updates a session with the specified `chat_id`.
    ///
    /// Returns `None` if there wasn't such a session, `Some(session)` if a
    /// `session` was updated.
    async fn update_session(
        &mut self,
        chat_id: i64,
        session: Session,
    ) -> Option<Session>;
}
