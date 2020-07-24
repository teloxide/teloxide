use teloxide::prelude::*;

use super::states::*;

#[macro_export]
macro_rules! extract_text {
    ($cx:ident) => {
        match $cx.update.text_owned() {
            Some(text) => text,
            None => {
                $cx.answer_str("Please, send me a text message").await?;
                return next(StartState);
            }
        }
    };
}

pub type Out = TransitionOut<Dialogue>;

pub async fn start(cx: TransitionIn, state: StartState) -> Out {
    let text = extract_text!(cx);

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

pub async fn have_number(cx: TransitionIn, state: HaveNumberState) -> Out {
    let text = extract_text!(cx);
    let num = state.number;

    if text.starts_with("/get") {
        cx.answer_str(format!("Here is your number: {}", num)).await?;
        next(state)
    } else if text.starts_with("/reset") {
        cx.answer_str("Resetted number").await?;
        next(StartState)
    } else {
        cx.answer_str("Please, send /get or /reset").await?;
        next(state)
    }
}
