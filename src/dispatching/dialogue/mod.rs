//! Dealing with dialogues.
//!
//! There are three main components:
//!
//!  1. Your type `D` (typically an enumeration), implementing [`Transition`].
//! It is essentially a [FSM]: its variants are possible dialogue states and
//! [`Transition::react`] is a transition function.
//!
//!  2. State types, forming `D`. They implement [`Subtransition`].
//!
//!  2. [`Storage<D>`], which encapsulates all the dialogues.
//!
//!  3. [`DialogueDispatcher`], which encapsulates your handler, [`Storage<D>`],
//! and implements [`DispatcherHandler`].
//!
//! You pass [`DialogueDispatcher`] into [`Dispatcher`]. Every time
//! [`Dispatcher`] sees an incoming input, it is transferred into
//! [`DialogueDispatcher`], and the following steps are executed:
//!
//!  1. If a storage doesn't contain a dialogue from this chat, supply
//! `D::default()` into you handler, otherwise, supply the saved dialogue
//! from this chat.
//!  2. If a handler has returned [`DialogueStage::Exit`], remove the dialogue
//! from the storage, otherwise ([`DialogueStage::Next`]) force the storage to
//! update the dialogue.
//!
//! To avoid boilerplate, teloxide exports these convenient things: the [`next`]
//! and [`exit`] functions, and `#[derive(BotDialogue)]` with
//! `#[teloxide(subtransition)]`. Here's how your dialogues management code
//! skeleton should look like:
//!
//! ```no_run
//! # #[cfg(feature = "macros")] {
//! use std::convert::Infallible;
//!
//! use teloxide::{
//!     dispatching::dialogue::{InMemStorageError, Transition},
//!     prelude::*,
//!     teloxide, RequestError,
//! };
//!
//! #[derive(Clone)]
//! struct _1State;
//! #[derive(Clone)]
//! struct _2State;
//! #[derive(Clone)]
//! struct _3State;
//!
//! type Out = TransitionOut<D, RequestError>;
//!
//! #[teloxide(subtransition)]
//! async fn _1_transition(_state: _1State, _cx: TransitionIn<AutoSend<Bot>>) -> Out {
//!     todo!()
//! }
//!
//! #[teloxide(subtransition)]
//! async fn _2_transition(_state: _2State, _cx: TransitionIn<AutoSend<Bot>>) -> Out {
//!     todo!()
//! }
//!
//! #[teloxide(subtransition)]
//! async fn _3_transition(_state: _3State, _cx: TransitionIn<AutoSend<Bot>>) -> Out {
//!     todo!()
//! }
//!
//! #[derive(Clone, Transition)]
//! enum D {
//!     _1(_1State),
//!     _2(_2State),
//!     _3(_3State),
//! }
//!
//! impl Default for D {
//!     fn default() -> Self {
//!         Self::_1(_1State)
//!     }
//! }
//!
//! type In = DialogueWithCx<AutoSend<Bot>, Message, D, InMemStorageError>;
//!
//! #[tokio::main]
//! async fn main() {
//!     run().await;
//! }
//!
//! async fn run() {
//!     teloxide::enable_logging!();
//!     log::info!("Starting dialogue_bot!");
//!
//!     let bot = Bot::from_env().auto_send();
//!
//!     Dispatcher::new(bot)
//!         .messages_handler(DialogueDispatcher::new(
//!             |DialogueWithCx { cx, dialogue }: In| async move {
//!                 // No panic because of std::convert::Infallible.
//!                 let dialogue = dialogue.unwrap();
//!                 dialogue
//!                     // Instead of () you can pass an arbitrary value, see below.
//!                     .react(cx, ())
//!                     .await
//!                     .expect("Something wrong with the bot!")
//!             },
//!         ))
//!         .dispatch()
//!         .await;
//! }
//! # }
//! ```
//!
//!  - `#[teloxide(subtransition)]` implements [`Subtransition`] for the first
//!    argument of a function.
//!  - `#[derive(Transition)]` implements [`Transition`] for `D`, if all the
//!    variants implement [`Subtransition`].
//!
//! `()` in `.react(cx, ())` is an arbitrary value, which you can pass into
//! Subtransitions. Just append `ans: T` to the parameters of the
//! Subtransitions to pass a differen type.
//!
//! See [examples/dialogue_bot] as a real example.
//!
//! [`Transition`]: crate::dispatching::dialogue::Transition
//! [`Subtransition`]: crate::dispatching::dialogue::Subtransition
//! [`Transition::react`]:
//! crate::dispatching::dialogue::Transition::react
//! [FSM]: https://en.wikipedia.org/wiki/Finite-state_machine
//!
//! [`Storage<D>`]: crate::dispatching::dialogue::Storage
//!
//! [`DialogueStage<D>`]: crate::dispatching::dialogue::DialogueStage
//! [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
//!
//! [`DialogueStage::Exit`]:
//! crate::dispatching::dialogue::DialogueStage::Exit
//! [`DialogueStage::Next`]: crate::dispatching::dialogue::DialogueStage::Next
//!
//! [`up!`]: crate::up
//! [`next`]: crate::dispatching::dialogue::next
//! [`exit`]: crate::dispatching::dialogue::exit
//!
//! [`DispatcherHandler`]: crate::dispatching::DispatcherHandler
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [`Dispatcher::messages_handler`]:
//! crate::dispatching::Dispatcher::messages_handler
//! [`UpdateKind::Message(message)`]: crate::types::UpdateKind::Message
//!
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/tree/master/examples/dialogue_bot

#![allow(clippy::type_complexity)]

mod dialogue_dispatcher;
mod dialogue_dispatcher_handler;
mod dialogue_stage;
mod dialogue_with_cx;
mod get_chat_id;
mod storage;
mod transition;

pub use dialogue_dispatcher::DialogueDispatcher;
pub use dialogue_dispatcher_handler::DialogueDispatcherHandler;
pub use dialogue_stage::{exit, next, DialogueStage};
pub use dialogue_with_cx::DialogueWithCx;
pub use get_chat_id::GetChatId;
pub use transition::{
    Subtransition, SubtransitionOutputType, Transition, TransitionIn, TransitionOut,
};

#[cfg(feature = "macros")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
pub use teloxide_macros::Transition;

#[cfg(feature = "redis-storage")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "redis-storage")))]
pub use storage::{RedisStorage, RedisStorageError};

#[cfg(feature = "sqlite-storage")]
pub use storage::{SqliteStorage, SqliteStorageError};

pub use storage::{serializer, InMemStorage, InMemStorageError, Serializer, Storage, TraceStorage};
