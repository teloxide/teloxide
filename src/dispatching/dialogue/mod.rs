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
//! implements [`CtxHandler`].
//!
//! You supply [`SessionDispatcher`] into [`Dispatcher`]. Every time
//! [`Dispatcher`] calls `SessionDispatcher::handle_ctx(...)`, the following
//! steps are executed:
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
//! [`CtxHandler`]: crate::dispatching::CtxHandler
//! [`Dispatcher`]: crate::dispatching::Dispatcher

// TODO: examples

#![allow(clippy::module_inception)]
#![allow(clippy::type_complexity)]

mod dialogue;
mod dialogue_dispatcher;
mod dialogue_handler_ctx;
mod dialogue_stage;
mod get_chat_id;
mod storage;

pub use dialogue::Dialogue;
pub use dialogue_dispatcher::DialogueDispatcher;
pub use dialogue_handler_ctx::DialogueHandlerCtx;
pub use dialogue_stage::{exit, next, DialogueStage};
pub use get_chat_id::GetChatId;
pub use storage::{InMemStorage, Storage};
