//! Dealing with dialogues.
//!
//! There are four main components:
//!
//!  1. Your session type `Session`, which designates a dialogue state at the
//! current moment.
//!  2. [`Storage`], which encapsulates all the sessions.
//!  3. Your handler, which receives an update and turns your session into the
//! next state.
//! 4. [`SessionDispatcher`], which encapsulates your handler, [`Storage`], and
//! implements [`AsyncHandler`].
//!
//! You supply [`SessionDispatcher`] into [`Dispatcher`]. Every time
//! [`Dispatcher`] calls `SessionDispatcher::handle(...)`, the following steps
//! are executed:
//!
//!  1. If a storage doesn't contain a session from this chat, supply
//! `Session::default()` into you handler, otherwise, supply the saved session
//! from this chat.
//!  3. If a handler has returned [`SessionState::Exit`], remove the session
//! from the storage, otherwise ([`SessionState::Next`]) force the storage to
//! update the session.
//!
//! [`Storage`]: crate::dispatching::session::Storage
//! [`SessionDispatcher`]: crate::dispatching::session::SessionDispatcher
//! [`SessionState::Exit`]:
//! crate::dispatching::session::SessionState::Exit
//! [`SessionState::Next`]: crate::dispatching::session::SessionState::Next
//! [`AsyncHandler`]: crate::dispatching::AsyncHandler
//! [`Dispatcher`]: crate::dispatching::Dispatcher

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
