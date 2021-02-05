use crate::dialogue::{states::receive_age::ReceiveAgeState, Dialogue, SubIn, MyDialogue};
use teloxide::prelude::*;
use teloxide_macros::teloxide;

#[derive(Debug, Clone, Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: SubIn<ReceiveFullNameState>,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut {
    cx.answer_str("How old are you?").await?;
    state.next(move |prev| ReceiveAgeState::up(prev, ans)).await;
    Ok(())
}
