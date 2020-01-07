use super::storage::{InMemStorage, Storage};
use crate::{
    dispatching::{Handler, SessionState},
    types::{ChatKind, Update, UpdateKind},
};

pub struct Dispatcher<'a, S, H> {
    storage: Box<dyn Storage<Session = S> + 'a>,
    handler: H,
}

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum DispatchResult {
    Handled,
    Unhandled,
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

impl<'a, S, H> Dispatcher<'a, S, H>
where
    S: Default + 'a,
    H: Handler<S>,
{
    pub fn new(handler: H) -> Self {
        Self {
            storage: Box::new(InMemStorage::default()),
            handler,
        }
    }

    pub fn with_storage<Stg>(handler: H, storage: Stg) -> Self
    where
        Stg: Storage<Session = S> + 'a,
    {
        Self {
            storage: Box::new(storage),
            handler,
        }
    }

    pub async fn save_storage(&mut self) {
        self.storage.save().await;
    }

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

impl<'a, S, H> Drop for Dispatcher<'a, S, H> {
    fn drop(&mut self) {
        // TODO: run self.save_storage()
    }
}
