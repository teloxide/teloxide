use teloxide::prelude::*;

use super::states::*;

pub type Cx = UpdateWithCx<Message>;
pub type Out = TransitionOut<Dialogue>;

async fn start(cx: Cx, state: StartState, text: &str) -> Out {
    if let Ok(number) = text.parse() {
        cx.answer_str(format!(
            "Remembered number {}. Now use /get or /reset",
            number
        ))
        .await?;
        next(state.up(number))
    } else {
        cx.answer_str("Please, send me a number").await?;
        next(state)
    }
}

async fn have_number(cx: Cx, state: HaveNumberState, text: &str) -> Out {
    let num = state.number;

    if text.starts_with("/get") {
        cx.answer_str(format!("Here is your number: {}", num)).await?;
        next(state)
    } else if text.starts_with("/reset") {
        cx.answer_str(format!("Resetted number")).await?;
        next(StartState)
    } else {
        cx.answer_str("Please, send /get or /reset").await?;
        next(state)
    }
}

pub async fn dispatch(cx: Cx, dialogue: Dialogue, text: &str) -> Out {
    match dialogue {
        Dialogue::Start(state) => start(cx, state, text).await,
        Dialogue::HaveNumber(state) => have_number(cx, state, text).await,
    }
}
