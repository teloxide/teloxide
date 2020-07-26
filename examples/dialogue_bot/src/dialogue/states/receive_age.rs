use crate::dialogue::{
    states::receive_location::ReceiveLocationState, Dialogue,
};
use teloxide::prelude::*;

#[derive(Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

#[teloxide(transition)]
async fn receive_age_state(
    state: ReceiveAgeState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.parse::<u8>() {
        Ok(ans) => {
            cx.answer_str("What's your location?").await?;
            next(ReceiveLocationState::up(state, ans))
        }
        _ => {
            cx.answer_str("Send me a number.").await?;
            next(state)
        }
    }
}
