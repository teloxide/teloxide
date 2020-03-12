//! Dealing with dialogues.
//!
//! There are four main components:
//!
//!  1. Your type `D`, which designates a dialogue state at the current
//! moment.
//!  2. [`Storage<D>`], which encapsulates all the dialogues.
//!  3. Your handler, which receives an update and turns your dialogue into the
//! next state ([`DialogueDispatcherHandlerCx<YourUpdate, D>`] ->
//! [`DialogueStage<D>`]).
//!  4. [`DialogueDispatcher`], which encapsulates your handler, [`Storage<D>`],
//! and implements [`DispatcherHandler`].
//!
//! For example, you supply [`DialogueDispatcher`] into
//! [`Dispatcher::messages_handler`]. Every time [`Dispatcher`] sees an incoming
//! [`UpdateKind::Message(message)`], `message` is transferred into
//! [`DialogueDispatcher`]. After this, following steps are executed:
//!
//!  1. If a storage doesn't contain a dialogue from this chat, supply
//! `D::default()` into you handler, otherwise, supply the saved dialogue
//! from this chat.
//!  2. If a handler has returned [`DialogueStage::Exit`], remove the dialogue
//! from the storage, otherwise ([`DialogueStage::Next`]) force the storage to
//! update the dialogue.
//!
//! Please, see [examples/dialogue_bot] as an example.
//!
//! [`Storage<D>`]: crate::dispatching::dialogue::Storage
//! [`DialogueStage<D>`]: crate::dispatching::dialogue::DialogueStage
//! [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
//! [`DialogueStage::Exit`]:
//! crate::dispatching::dialogue::DialogueStage::Exit
//! [`DialogueStage::Next`]: crate::dispatching::dialogue::DialogueStage::Next
//! [`DispatcherHandler`]: crate::dispatching::DispatcherHandler
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [`Dispatcher::messages_handler`]:
//! crate::dispatching::Dispatcher::messages_handler
//! [`UpdateKind::Message(message)`]: crate::types::UpdateKind::Message
//! [`DialogueDispatcherHandlerCx<YourUpdate, D>`]:
//! crate::dispatching::dialogue::DialogueDispatcherHandlerCx
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/tree/master/examples/dialogue_bot

#![allow(clippy::type_complexity)]

mod dialogue_dispatcher;
mod dialogue_dispatcher_handler;
mod dialogue_dispatcher_handler_cx;
mod dialogue_stage;
mod get_chat_id;
mod storage;

pub use dialogue_dispatcher::DialogueDispatcher;
pub use dialogue_dispatcher_handler::DialogueDispatcherHandler;
pub use dialogue_dispatcher_handler_cx::DialogueDispatcherHandlerCx;
pub use dialogue_stage::{exit, next, DialogueStage};
pub use get_chat_id::GetChatId;
#[cfg(feature = "redis-storage")]
pub use storage::RedisStorage;
pub use storage::{InMemStorage, Serializer, Storage};
