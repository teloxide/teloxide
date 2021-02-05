use crate::{
    dispatching::UpdateWithCx,
    types::Message,
};
use futures::future::BoxFuture;
use crate::dispatching::dialogue::Dialogue;

/// Represents a transition function of a dialogue FSM.
pub trait Transition: Sized {
    type Aux;
    type Error;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` will be passed to each subtransition function.
    fn react(
        self,
        cx: TransitionIn,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Error>>;
}

/// Like [`Transition`], but from `StateN` -> `Dialogue`.
///
/// [`Transition`]: crate::dispatching::dialogue::Transition
pub trait Subtransition<S>: Sized {
    type Aux;
    type Error;

    /// Turns itself into another state, depending on the input message.
    ///
    /// `aux` is something that is provided by the call side, for example,
    /// message's text.
    fn react(
        self,
        cx: TransitionIn,
        aux: Self::Aux,
    ) -> BoxFuture<'static, TransitionOut<Self::Error>>;
}

pub trait SubtransitionState {
    type State;
    type StorageError;
    type Dialogue;
}

impl<D, E, Cur> SubtransitionState for Dialogue<D, E, Cur> {
    type State = Cur;
    type StorageError = E;
    type Dialogue = D;
}

/// A type returned from a FSM subtransition function.
///
/// Now it is used only inside `#[teloxide(subtransition)]` for type inference.
pub trait SubtransitionOutputType {
    type Error;
}

impl<E> SubtransitionOutputType for TransitionOut<E> {
    type Error = E;
}

/// An input passed into a FSM (sub)transition function.
pub type TransitionIn = UpdateWithCx<Message>;

pub type TransitionOut<E = crate::RequestError> = Result<(), E>;
