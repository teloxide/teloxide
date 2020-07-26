use crate::dialogue::{states::receive_age::ReceiveAgeState, Dialogue};
use teloxide::prelude::*;

#[derive(Generic)]
pub struct ReceiveFullNameState;

#[teloxide(transition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str("How old are you?").await?;
    next(ReceiveAgeState::up(state, ans))
}
