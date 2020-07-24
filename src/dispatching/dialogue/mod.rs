//! Dealing with dialogues.
//!
//! There are three main components:
//!
//!  1. Your type `D` (typically an enumeration), implementing [`BotDialogue`].
//! It is essentially a [FSM]: its variants are possible dialogue states and
//! [`BotDialogue::dispatch`] is a transition function.
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
//! To avoid boilerplate, teloxide exports these convenient things: the [`up!`]
//! macro, the [`next`] and [`exit`] functions, and `#[derive(BotDialogue)]`.
//! Here's how your dialogues management code skeleton should look like:
//!
//! ```no_run
//! use std::convert::Infallible;
//!
//! use teloxide::prelude::*;
//! use teloxide_macros::BotDialogue;
//!
//! struct _1State;
//! struct _2State;
//! struct _3State;
//!
//! pub type Out = TransitionOut<D>;
//!
//! pub async fn _1_transition(cx: TransitionIn, state: _1State) {
//!     todo!()
//! }
//! pub async fn _2_transition(cx: TransitionIn, state: _2State) {
//!     todo!()
//! }
//! pub async fn _3_transition(cx: TransitionIn, state: _3State) {
//!     todo!()
//! }
//!
//! #[derive(BotDialogue)]
//! enum D {
//!     #[transition(_1_transition)]
//!     _1(_1State),
//!     #[transition(_2_transition)]
//!     _2(_2State),
//!     #[transition(_3_transition)]
//!     _3(_3State),
//! }
//!
//! impl Default for D {
//!     fn default() -> Self {
//!         Self::_1(_1State)
//!     }
//! }
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
//!             |input: DialogueWithCx<Message, D, Infallible>| async move {
//!                 // Unwrap without panic because of std::convert::Infallible.
//!                 input
//!                     .dialogue
//!                     .unwrap()
//!                     .dispatch(input.cx)
//!                     .await
//!                     .expect("Something wrong with the bot!")
//!             },
//!         ))
//!         .dispatch()
//!         .await;
//! }
//! ```
//!
//! See [examples/dialogue_bot] as a real example.
//!
//! [`BotDialogue`]: crate::dispatching::dialogue::BotDialogue
//! [`BotDialogue::dispatch`]:
//! crate::dispatching::dialogue::BotDialogue::dispatch
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

mod bot_dialogue;
mod dialogue_dispatcher;
mod dialogue_dispatcher_handler;
mod dialogue_stage;
mod dialogue_with_cx;
mod get_chat_id;
mod storage;

use crate::{requests::ResponseResult, types::Message};
pub use bot_dialogue::BotDialogue;
pub use dialogue_dispatcher::DialogueDispatcher;
pub use dialogue_dispatcher_handler::DialogueDispatcherHandler;
pub use dialogue_stage::{exit, next, DialogueStage};
pub use dialogue_with_cx::DialogueWithCx;
pub use get_chat_id::GetChatId;

#[cfg(feature = "redis-storage")]
pub use storage::{RedisStorage, RedisStorageError};

use crate::dispatching::UpdateWithCx;
pub use storage::{serializer, InMemStorage, Serializer, Storage};

/// Generates `.up(field)` methods for dialogue states.
///
/// Given inductively defined states, this macro generates `.up(field)` methods
/// from `Sn` to `Sn+1`.
///
/// # Examples
/// ```
/// use teloxide::prelude::*;
///
/// struct StartState;
///
/// struct ReceiveWordState {
///     rest: StartState,
/// }
///
/// struct ReceiveNumberState {
///     rest: ReceiveWordState,
///     word: String,
/// }
///
/// struct ExitState {
///     rest: ReceiveNumberState,
///     number: i32,
/// }
///
/// up!(
///     StartState -> ReceiveWordState,
///     ReceiveWordState + [word: String] -> ReceiveNumberState,
///     ReceiveNumberState + [number: i32] -> ExitState,
/// );
///
/// let start_state = StartState;
/// let receive_word_state = start_state.up();
/// let receive_number_state = receive_word_state.up("Hello".to_owned());
/// let exit_state = receive_number_state.up(123);
/// ```
#[macro_export]
macro_rules! up {
    ( $( $from:ident $(+ [$field_name:ident : $field_type:ty])? -> $to:ident ),+, ) => {
        $(
            impl $from {
                pub fn up(self, $( $field_name: $field_type )?) -> $to {
                    $to { rest: self, $($field_name)? }
                }
            }
        )+
    };
}

/// An input passed into a FSM transition function.
pub type TransitionIn = UpdateWithCx<Message>;

/// A type returned from a FSM transition function.
pub type TransitionOut<D> = ResponseResult<DialogueStage<D>>;
