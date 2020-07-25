use crate::states::{
    Dialogue, ReceiveAgeState, ReceiveFullNameState, ReceiveLocationState,
    StartState,
};

use teloxide::prelude::*;

pub type Out = TransitionOut<Dialogue>;

#[teloxide(transition)]
async fn start(_state: StartState, cx: TransitionIn) -> Out {
    cx.answer_str("Let's start! What's your full name?").await?;
    next(ReceiveFullNameState)
}

#[teloxide(transition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn,
) -> Out {
    match cx.update.text_owned() {
        Some(ans) => {
            cx.answer_str("How old are you?").await?;
            next(ReceiveAgeState::up(state, ans))
        }
        _ => {
            cx.answer_str("Send me a text message.").await?;
            next(state)
        }
    }
}

#[teloxide(transition)]
async fn receive_age_state(state: ReceiveAgeState, cx: TransitionIn) -> Out {
    match cx.update.text().map(str::parse::<u8>) {
        Some(Ok(ans)) => {
            cx.answer_str("What's your location?").await?;
            next(ReceiveLocationState::up(state, ans))
        }
        _ => {
            cx.answer_str("Send me a number.").await?;
            next(state)
        }
    }
}

#[teloxide(transition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn,
) -> Out {
    match cx.update.text() {
        Some(ans) => {
            cx.answer_str(format!(
                "Full name: {}\nAge: {}\nLocation: {}",
                state.full_name, state.age, ans
            ))
            .await?;
            exit()
        }
        _ => {
            cx.answer_str("Send me a text message.").await?;
            next(state)
        }
    }
}
