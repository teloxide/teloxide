use super::{
    super::DispatchResult,
    storage::{InMemStorage, Storage},
};
use crate::{
    dispatching::{Handler, SessionState},
    types::{ChatKind, Update, UpdateKind},
};

/// A dispatcher that dispatches updates from 1-to-1 chats.
pub struct Dispatcher<'a, Session, H> {
    storage: Box<dyn Storage<Session> + 'a>,
    handler: H,
}

#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! private_chat_id {
        ($msg:expr) => {
            match &$msg.chat.kind {
                ChatKind::Private { .. } => $msg.chat.id,
                _ => return DispatchResult::Unhandled,
            }
        };
    }
}

impl<'a, Session, H> Dispatcher<'a, Session, H>
where
    Session: Default + 'a,
    H: Handler<Session>,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::private::InMemStorage
    pub fn new(handler: H) -> Self {
        Self {
            storage: Box::new(InMemStorage::default()),
            handler,
        }
    }

    /// Creates a dispatcher with the specified `handler` and `storage`.
    pub fn with_storage<Stg>(handler: H, storage: Stg) -> Self
    where
        Stg: Storage<Session> + 'a,
    {
        Self {
            storage: Box::new(storage),
            handler,
        }
    }

    /// Dispatches a single `update`.
    ///
    /// ## Returns
    /// Returns [`DispatchResult::Handled`] if `update` was supplied to a
    /// handler, and [`DispatchResult::Unhandled`] if it was an update not
    /// from a 1-to-1 chat.
    ///
    /// [`DispatchResult::Handled`]: crate::dispatching::DispatchResult::Handled
    /// [`DispatchResult::Unhandled`]:
    /// crate::dispatching::DispatchResult::Unhandled
    pub async fn dispatch(&mut self, update: Update) -> DispatchResult {
        let chat_id = match &update.kind {
            UpdateKind::Message(msg) => private_chat_id!(msg),
            UpdateKind::EditedMessage(msg) => private_chat_id!(msg),
            _ => return DispatchResult::Unhandled,
        };

        let session = self
            .storage
            .remove_session(chat_id)
            .await
            .unwrap_or_default();

        if let SessionState::Continue(session) =
            self.handler.handle(session, update).await
        {
            if self
                .storage
                .update_session(chat_id, session)
                .await
                .is_some()
            {
                panic!(
                    "We previously storage.remove_session() so \
                     storage.update_session() must return None"
                );
            }
        }

        DispatchResult::Handled
    }
}
