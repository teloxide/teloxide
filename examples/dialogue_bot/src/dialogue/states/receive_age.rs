use crate::dialogue::{states::receive_location::ReceiveLocationState, Dialogue};
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

#[teloxide(subtransition)]
async fn receive_age_state(
    state: ReceiveAgeState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.parse::<u8>() {
        Ok(ans) => {
            cx.answer("What's your location?").await?;
            next(ReceiveLocationState::up(state, ans))
        }
        _ => {
            cx.answer("Send me a number.").await?;
            next(state)
        }
    }
}
