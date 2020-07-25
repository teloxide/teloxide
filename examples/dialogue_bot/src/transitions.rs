use crate::states::{
    Dialogue, Receive10x5AnswerState, ReceiveDaysOfWeekState,
    ReceiveGandalfAlternativeNameState, StartState,
};

use teloxide::prelude::*;
use teloxide_macros::teloxide;

pub type Out = TransitionOut<Dialogue>;

#[teloxide(transition)]
async fn start(state: StartState, cx: TransitionIn) -> Out {
    cx.answer_str("Let's start our test! How many days per week are there?")
        .await?;
    next(ReceiveDaysOfWeekState)
}

#[teloxide(transition)]
async fn receive_days_of_week(
    state: ReceiveDaysOfWeekState,
    cx: TransitionIn,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 7 => {
            cx.answer_str("10*5 = ?").await?;
            next(append_field(state, ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

#[teloxide(transition)]
async fn receive_10x5_answer(
    state: Receive10x5AnswerState,
    cx: TransitionIn,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 50 => {
            cx.answer_str("What's an alternative name of Gandalf?").await?;
            next(append_field(state, ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

#[teloxide(transition)]
async fn receive_gandalf_alternative_name(
    state: ReceiveGandalfAlternativeNameState,
    cx: TransitionIn,
) -> Out {
    match cx.update.text() {
        Some(ans) if ans == "Mithrandir" => {
            cx.answer_str(
                "Congratulations! You've successfully passed the test!",
            )
            .await?;
            exit()
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}
