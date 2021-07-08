use crate::dialogue::{states::ReceiveFullNameState, Dialogue};
use teloxide::prelude::*;

#[derive(Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(
    _state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    _ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("Let's start! What's your full name?").await?;
    next(ReceiveFullNameState)
}
