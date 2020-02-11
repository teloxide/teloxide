//! Dealing with dialogues.
//!
//! There are four main components:
//!
//!  1. Your type `State`, which designates a dialogue state at the current
//! moment.
//!  2. Your type `T`, which represents dialogue data.
//!  3. [`Dialogue`], which encapsulates the two types, described above.
//!  4. [`Storage`], which encapsulates all the sessions.
//!  5. Your handler, which receives an update and turns your session into the
//! next state.
//! 6. [`DialogueDispatcher`], which encapsulates your handler, [`Storage`], and
//! implements [`CtxHandler`].
//!
//! You supply [`DialogueDispatcher`] into [`Dispatcher`]. Every time
//! [`Dispatcher`] calls `DialogueDispatcher::handle_ctx(...)`, the following
//! steps are executed:
//!
//!  1. If a storage doesn't contain a dialogue from this chat, supply
//! `Dialogue::default()` into you handler, otherwise, supply the saved session
//! from this chat.
//!  3. If a handler has returned [`DialogueStage::Exit`], remove the session
//! from the storage, otherwise ([`DialogueStage::Next`]) force the storage to
//! update the session.
//!
//! Please, see https://github.com/teloxide/teloxide/tree/dev/examples/simple_dialogue.
//!
//! [`Storage`]: crate::dispatching::session::Storage
//! [`DialogueDispatcher`]: crate::dispatching::session::SessionDispatcher
//! [`DialogueStage::Exit`]:
//! crate::dispatching::dialogue::DialogueStage::Exit
//! [`DialogueStage::Next`]: crate::dispatching::dialogue::DialogueStage::Next
//! [`CtxHandler`]: crate::dispatching::CtxHandler
//! [`Dispatcher`]: crate::dispatching::Dispatcher

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
