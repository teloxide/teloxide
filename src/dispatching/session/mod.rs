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
//! [`Storage`]: crate::dispatching::session::Storage
//! [`SessionDispatcher`]: crate::dispatching::session::SessionDispatcher
//! [`dispatch(Bot, Upd)`]:
//! crate::dispatching::session::SessionDispatcher::dispatch
//! [`SessionState::Terminate`]:
//! crate::dispatching::session::SessionState::Terminate

// TODO: examples

mod get_chat_id;
mod session_dispatcher;
mod session_handler_ctx;
mod session_state;
mod storage;

pub use get_chat_id::GetChatId;
pub use session_dispatcher::SessionDispatcher;
pub use session_handler_ctx::SessionHandlerCtx;
pub use session_state::SessionState;
pub use storage::{InMemStorage, Storage};
