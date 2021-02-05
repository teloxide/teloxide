use crate::dialogue::{states::ReceiveFullNameState, SubIn, MyDialogue};
use teloxide::prelude::*;
use teloxide::dispatching::dialogue::{TransitionIn, TransitionOut};
use teloxide_macros::teloxide;

#[derive(Debug, Clone)]
pub struct StartState;

#[teloxide(subtransition)]
async fn start(state: SubIn<StartState>, cx: TransitionIn, _ans: String) -> TransitionOut {
    state.next(|_| ReceiveFullNameState).await;
    cx.answer_str("Let's start! What's your full name?").await?;
    Ok(())
}
