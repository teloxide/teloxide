use crate::dispatching::dialogue::{TransitionIn, TransitionOut};
use futures::future::BoxFuture;

/// Represents a dialogue FSM.
pub trait BotDialogue: Default {
    /// Turns itself into another state, depending on the input message.
    fn dispatch(
        self,
        cx: TransitionIn,
    ) -> BoxFuture<'static, TransitionOut<Self>>;
}
