use crate::dialogue::Dialogue;
use teloxide::prelude::*;
use teloxide_macros::teloxide;

#[derive(Generic)]
pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}

#[teloxide(transition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str(format!("Full name: {}\nAge: {}\nLocation: {}", state.full_name, state.age, ans))
        .await?;
    exit()
}
