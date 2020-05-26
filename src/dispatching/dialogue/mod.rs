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
//! [`DialogueWithCx<YourUpdate, D>`]:
//! crate::dispatching::dialogue::DialogueWithCx
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/tree/master/examples/dialogue_bot

#![allow(clippy::type_complexity)]

mod dialogue_dispatcher;
mod dialogue_dispatcher_handler;
mod dialogue_stage;
mod dialogue_with_cx;
mod get_chat_id;
mod storage;

use crate::{requests::ResponseResult, types::Message};
pub use dialogue_dispatcher::DialogueDispatcher;
pub use dialogue_dispatcher_handler::DialogueDispatcherHandler;
pub use dialogue_stage::{exit, next, DialogueStage, DialogueWrapper};
pub use dialogue_with_cx::DialogueWithCx;
pub use get_chat_id::GetChatId;
pub use storage::{InMemStorage, Storage};

/// Dispatches a dialogue state into transition functions.
///
/// # Example
/// ```no_run
/// use teloxide::prelude::*;
///
/// pub struct StartState;
/// pub struct ReceiveWordState;
/// pub struct ReceiveNumberState;
/// pub struct ExitState;
///
/// pub type Dialogue = Coprod!(
///     StartState,
///     ReceiveWordState,
///     ReceiveNumberState,
/// );
///
/// wrap_dialogue!(
///     Wrapper(Dialogue),
///     default Self(Dialogue::inject(StartState)),
/// );
///
/// pub type In<State> = TransitionIn<State, std::convert::Infallible>;
/// pub type Out = TransitionOut<Wrapper>;
///
/// pub async fn start(cx: In<StartState>) -> Out { todo!() }
/// pub async fn receive_word(cx: In<ReceiveWordState>) -> Out { todo!() }
/// pub async fn receive_number(cx: In<ReceiveNumberState>) -> Out { todo!() }
///
/// # #[tokio::main]
/// # async fn main() {
/// let cx: In<Dialogue> = todo!();
/// let (cx, dialogue) = cx.unpack();
///
/// // StartState -> start
/// // ReceiveWordState -> receive_word
/// // ReceiveNumberState -> receive_number
/// let stage = dispatch!(
///     [cx, dialogue] ->
///     [start, receive_word, receive_number]
/// );
/// # }
/// ```
#[macro_export]
macro_rules! dispatch {
    ([$cx:ident, $dialogue:ident] -> [$transition:ident, $($transitions:ident),+]) => {
        match $dialogue {
            Coproduct::Inl(state) => {
                $transition(teloxide::dispatching::dialogue::DialogueWithCx::new($cx, state)).await
            }
            Coproduct::Inr(another) => { dispatch!([$cx, another] -> [$($transitions),+]) }
        }
    };

    ([$cx:ident, $dialogue:ident] -> [$transition:ident]) => {
        match $dialogue {
            Coproduct::Inl(state) => {
                $transition(teloxide::dispatching::dialogue::DialogueWithCx::new($cx, state)).await
            }
            Coproduct::Inr(_absurd) => unreachable!(),
        }
    };
}

/// Generates a dialogue wrapper and implements `Default` for it.
///
/// The reason is to bypass orphan rules to be able to pass a user-defined
/// dialogue into [`DialogueDispatcher`]. Since a dialogue is
/// [`frunk::Coproduct`], we cannot directly satisfy the `D: Default`
/// constraint.
///
/// # Examples
/// ```
/// use teloxide::prelude::*;
///
/// struct StartState;
/// struct ReceiveWordState;
/// struct ReceiveNumberState;
/// struct ExitState;
///
/// type Dialogue = Coprod!(
///     StartState,
///     ReceiveWordState,
///     ReceiveNumberState,
/// );
///
/// wrap_dialogue!(
///     Wrapper(Dialogue),
///     default Self(Dialogue::inject(StartState)),
/// );
///
/// let start_state = Wrapper::default();
/// ```
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
/// [`frunk::Coproduct`]: https://docs.rs/frunk/0.3.1/frunk/coproduct/enum.Coproduct.html
#[macro_export]
macro_rules! wrap_dialogue {
    ($name:ident($dialogue:ident), default $default_block:expr, ) => {
        pub struct $name(pub $dialogue);

        impl teloxide::dispatching::dialogue::DialogueWrapper<$dialogue>
            for $name
        {
            fn new(d: $dialogue) -> Wrapper {
                $name(d)
            }
        }

        impl Default for $name {
            fn default() -> $name {
                $default_block
            }
        }
    };
}

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

/// A type passed into a FSM transition function.
pub type TransitionIn<State, E> = DialogueWithCx<Message, State, E>;

// A type returned from a FSM transition function.
pub type TransitionOut<DWrapper> = ResponseResult<DialogueStage<DWrapper>>;
