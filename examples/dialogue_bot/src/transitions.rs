use crate::states::{
    Dialogue, Receive10x5AnswerState, ReceiveDaysOfWeekState,
    ReceiveGandalfAlternativeNameState, StartState,
};
use teloxide::prelude::*;

pub type Cx = UpdateWithCx<Message>;
pub type Out = TransitionOut<Dialogue>;

pub async fn start(cx: Cx, state: StartState) -> Out {
    cx.answer_str("Let's start our test! How many days per week are there?")
        .await?;
    next(state.up())
}

pub async fn receive_days_of_week(
    cx: Cx,
    state: ReceiveDaysOfWeekState,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 7 => {
            cx.answer_str("10*5 = ?").await?;
            next(state.up(ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

pub async fn receive_10x5_answer(cx: Cx, state: Receive10x5AnswerState) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 50 => {
            cx.answer_str("What's an alternative name of Gandalf?").await?;
            next(state.up(ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

pub async fn receive_gandalf_alternative_name(
    cx: Cx,
    state: ReceiveGandalfAlternativeNameState,
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
