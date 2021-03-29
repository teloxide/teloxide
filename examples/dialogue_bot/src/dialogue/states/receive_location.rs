use crate::dialogue::Dialogue;
use teloxide::prelude::*;

#[derive(Clone, Generic)]
pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}

#[teloxide(subtransition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer(format!("Full name: {}\nAge: {}\nLocation: {}", state.full_name, state.age, ans))
        .await?;
    exit()
}
