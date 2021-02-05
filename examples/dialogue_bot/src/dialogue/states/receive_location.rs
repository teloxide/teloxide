use crate::dialogue::{SubIn, MyDialogue};
use teloxide::prelude::*;
use teloxide_macros::teloxide;

#[derive(Debug, Clone, Generic)]
pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}

#[teloxide(subtransition)]
async fn receive_location(
    state: SubIn<ReceiveLocationState>,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut {
    cx.answer_str(format!("Full name: {}\nAge: {}\nLocation: {}", state.data.full_name, state.data.age, ans))
        .await?;
    state.exit().await;
    Ok(())
}
