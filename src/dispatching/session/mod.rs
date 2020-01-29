//! Dispatching user sessions.
//!
//! There are four main components:
//!
//!  1. Your session type `Session`, which designates a dialogue state at the
//! current moment.
//!  2. [`Storage`] that encapsulates all the sessions.
//!  3. Your handler of type `H: async Fn(Session, Update) ->
//! SessionState<Session>` that receives an update and turns your session into
//! the next state.
//!  4. [`SessionDispatcher`], which encapsulates your handler and
//! [`Storage`], and has the [`dispatch(Bot, Upd)`] function.
//!
//! Every time you call `.dispatch(bot, update)` on your dispatcher, the
//! following steps are executed:
//!
//!  1. If a storage doesn't contain a session from this chat, supply
//! `Session::default()` into you handler, otherwise, supply the previous
//! session.
//!  3. If a handler has returned [`SessionState::Terminate`], remove the
//! session from a storage, otherwise force the storage to update the session.
//!
//! [`Storage`]: crate::dispatching::Storage
//! [`SessionDispatcher`]: crate::dispatching::SessionDispatcher
//! [`dispatch(Bot, Upd)`]:
//! crate::dispatching::SessionDispatcher::dispatch
//! [`SessionState::Terminate`]: crate::dispatching::SessionState::Terminate

// TODO: examples

mod get_chat_id;
mod storage;

use crate::{dispatching::Handler, Bot};
pub use get_chat_id::*;
pub use storage::*;

/// A context of a private message handler.
pub struct SessionHandlerCtx<'a, Upd, Session> {
    pub bot: &'a Bot,
    pub update: Upd,
    pub session: Session,
}

/// Continue or terminate a user session.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum SessionState<Session> {
    Continue(Session),
    Terminate,
}

/// A dispatcher of user sessions.
pub struct SessionDispatcher<'a, Session, H> {
    storage: Box<dyn Storage<Session> + 'a>,
    handler: H,
}

impl<'a, Session, H> SessionDispatcher<'a, Session, H>
where
    Session: Default + 'a,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::InMemStorage
    #[must_use]
    pub fn new(handler: H) -> Self {
        Self {
            storage: Box::new(InMemStorage::default()),
            handler,
        }
    }

    /// Creates a dispatcher with the specified `handler` and `storage`.
    #[must_use]
    pub fn with_storage<Stg>(handler: H, storage: Stg) -> Self
    where
        Stg: Storage<Session> + 'a,
    {
        Self {
            storage: Box::new(storage),
            handler,
        }
    }

    /// Dispatches a single `message` from a private chat.
    pub async fn dispatch<Upd>(&'a mut self, bot: &'a Bot, update: Upd)
    where
        H: Handler<SessionHandlerCtx<'a, Upd, Session>, SessionState<Session>>,
        Upd: GetChatId,
    {
        let chat_id = update.chat_id();

        let session = self
            .storage
            .remove_session(chat_id)
            .await
            .unwrap_or_default();

        if let SessionState::Continue(new_session) = self
            .handler
            .handle(SessionHandlerCtx {
                bot,
                update,
                session,
            })
            .await
        {
            if self
                .storage
                .update_session(chat_id, new_session)
                .await
                .is_some()
            {
                panic!(
                    "We previously storage.remove_session() so \
                     storage.update_session() must return None"
                );
            }
        }
    }
}
