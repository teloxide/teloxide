use super::{
    super::DispatchResult,
    storage::{InMemStorage, Storage},
};
use crate::{
    dispatching::{chat::ChatUpdate, Handler, SessionState},
    types::{Update, UpdateKind},
};
use crate::dispatching::chat::ChatUpdateKind;

/// A dispatcher that dispatches updates from chats.
pub struct Dispatcher<'a, Session, H> {
    storage: Box<dyn Storage<Session> + 'a>,
    handler: H,
}

impl<'a, Session, H> Dispatcher<'a, Session, H>
where
    Session: Default + 'a,
    H: Handler<Session, ChatUpdate>,
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
    /// from a chat.
    ///
    /// [`DispatchResult::Handled`]: crate::dispatching::DispatchResult::Handled
    /// [`DispatchResult::Unhandled`]:
    /// crate::dispatching::DispatchResult::Unhandled
    pub async fn dispatch(&mut self, update: Update) -> DispatchResult {
        let chat_update = match update.kind {
            UpdateKind::Message(msg) => ChatUpdate { id: update.id, kind: ChatUpdateKind::Message(msg) },
            UpdateKind::EditedMessage(msg) => ChatUpdate { id: update.id, kind: ChatUpdateKind::EditedMessage(msg) },
            UpdateKind::CallbackQuery(query) => ChatUpdate { id: update.id, kind: ChatUpdateKind::CallbackQuery(query) },
            _ => return DispatchResult::Unhandled,
        };

        let chat_id = match &chat_update.kind {
            ChatUpdateKind::Message(msg) => msg.chat.id,
            ChatUpdateKind::EditedMessage(msg) => msg.chat.id,
            ChatUpdateKind::CallbackQuery(query) => match &query.message {
                None => return DispatchResult::Unhandled,
                Some(msg) => msg.chat.id,
            },
        };

        let session = self
            .storage
            .remove_session(chat_id)
            .await
            .unwrap_or_default();

        if let SessionState::Continue(session) =
            self.handler.handle(session, chat_update).await
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
