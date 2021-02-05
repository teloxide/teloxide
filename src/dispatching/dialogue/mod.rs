//! Dealing with dialogues.
//!
//! Dialogues - mechanism allows you to have state for the dialogues like FSM and store it consistently.
//!
//! There are four main components:
//!
//!  1. Your type `D` (typically an enumeration), implementing [`Transition`].
//! It is essentially a [FSM]: its variants are possible dialogue states and
//! [`Transition::react`] is a transition function.
//!
//!  2. State types, forming `D`. They implement [`Subtransition`].
//!
//!  3. [`Storage<D>`], which encapsulates all the dialogues.
//!
//!  4. [`DialogueDispatcher`], which encapsulates your handlers, [`Storage<D>`] and [`Dispatcher`]
//!
//! You build [`DialogueDispatcher`] using [`DialogueDispatcherBuilder`] which incapsulate
//! [`Dispatcher`] and has it's methods like `handle` and `data`. Every time
//! [`DialogueDispatcher`] give an update, it tries to get chat_id of the update. If dispatcher get it,
//! it request [`Storage`] for a state of the chat with this chat_id. If a storage doesn't contain a dialogue
//! from this chat, supply `D::default()` into you handler, otherwise, supply the saved dialogue
//! from this chat.
//!
//! After you get a dialogue in the handler, you must to change state using [`Dialogue::next`](crate::dispatching::dialogue::Dialogue::next) or
//! [`Dialogue::exit`](crate::dispatching::dialogue::Dialogue::exit) methods.
//!
//! To avoid boilerplate, teloxide exports these convenient things: `#[derive(BotDialogue)]` with
//! `#[teloxide(subtransition)]`. Here's how your dialogues management code
//! skeleton should look like:
//!
//! ```no_run
//! # #[cfg(feature = "macros")] {
//! use std::convert::Infallible;
//!
//! use teloxide::{dispatching::dialogue::{Transition, Dialogue, TransitionOut, DialoguewWithCx}, prelude::*, teloxide};
//!
//! struct _1State;
//! struct _2State;
//! struct _3State;
//!
//! type SubIn<T> = Dialogue<D, Infallible, T>;
//!
//! #[teloxide(subtransition(_1State -> D))]
//! async fn _1_transition(_state: SubIn<_1State>, _cx: TransitionIn) -> TransitionOut {
//!     todo!()
//! }
//!
//! #[teloxide(subtransition(_2State -> D))]
//! async fn _2_transition(_state: SubIn<_2State>, _cx: TransitionIn) -> TransitionOut {
//!     todo!()
//! }
//!
//! #[teloxide(subtransition(_3State -> D))]
//! async fn _3_transition(_state: SubIn<_3State>, _cx: TransitionIn) -> TransitionOut {
//!     todo!()
//! }
//!
//! #[derive(Transition)]
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
//! type In = DialogueWithCx<Message, D, Infallible>;
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
//!     let bot = Bot::from_env();
//!
//!     Dispatcher::new(bot)
//!         .messages_handler(DialogueDispatcher::new(
//!             |DialogueWithCx { cx, dialogue }: In| async move {
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
//! [`Dispatcher`]: crate::dispatching::Dispatcher
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

mod dialogue_ctx;
mod dialogue_dispatcher;
mod dialogue_handler_builder_ext;
mod dialogue_with_cx;
mod get_chat_id;
mod storage;
mod transition;
mod dialogue;

pub use dialogue_dispatcher::{DialogueDispatcher, DialogueDispatcherBuilder};
pub use dialogue_handler_builder_ext::DialogueHandlerBuilderExt;
pub use dialogue_with_cx::DialogueWithCx;
pub use dialogue_ctx::DialogueContext;
pub use get_chat_id::GetChatId;
pub use transition::{
    Subtransition, Transition, TransitionIn, TransitionOut, SubtransitionOutputType, SubtransitionState, SubtransitionHack
};
pub use dialogue::Dialogue;

#[cfg(feature = "macros")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
pub use teloxide_macros::Transition;

#[cfg(feature = "redis-storage")]
// FIXME(waffle): use `docsrs` here when issue with combine is resolved <https://github.com/teloxide/teloxide/pull/305#issuecomment-716172103>
#[cfg_attr(all(teloxide_docsrs, feature = "nightly"), doc(cfg(feature = "redis-storage")))]
pub use storage::{RedisStorage, RedisStorageError};

#[cfg(feature = "sqlite-storage")]
pub use storage::{SqliteStorage, SqliteStorageError};

pub use storage::{serializer, InMemStorage, Serializer, Storage, TraceStorage};
