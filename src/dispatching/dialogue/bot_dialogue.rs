use crate::{
    dispatching::{dialogue::TransitionOut, UpdateWithCx},
    types::Message,
};
use futures::future::BoxFuture;

/// Represents a dialogue FSM.
pub trait BotDialogue: Default {
    /// Turns itself into another state, depending on the input message.
    fn dispatch(
        self,
        cx: UpdateWithCx<Message>,
    ) -> BoxFuture<'static, TransitionOut<Self>>;
}
