use crate::dialogue::{states::receive_location::ReceiveLocationState, Dialogue, SubIn, MyDialogue};
use teloxide::prelude::*;
use teloxide_macros::teloxide;

#[derive(Debug, Clone, Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

#[teloxide(subtransition)]
async fn receive_age_state(
    state: SubIn<ReceiveAgeState>,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut {
    match ans.parse::<u8>() {
        Ok(ans) => {
            state.next(|x| ReceiveLocationState::up(x, ans)).await;
            cx.answer_str("What's your location?").await?;
        }
        _ => {
            state.stay().await;
            cx.answer_str("Send me a number.").await?;
        }
    }
    Ok(())
}
