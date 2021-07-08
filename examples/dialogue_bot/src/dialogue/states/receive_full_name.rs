use crate::dialogue::{states::receive_age::ReceiveAgeState, Dialogue};
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer("How old are you?").await?;
    next(ReceiveAgeState::up(state, ans))
}
