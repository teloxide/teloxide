use teloxide::prelude::*;

use super::states::*;

#[teloxide(subtransition)]
async fn start(
    state: StartState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    if let Ok(number) = ans.parse() {
        cx.answer(format!("Remembered number {}. Now use /get or /reset", number)).await?;
        next(HaveNumberState { number })
    } else {
        cx.answer("Please, send me a number").await?;
        next(state)
    }
}

#[teloxide(subtransition)]
async fn have_number(
    state: HaveNumberState,
    cx: TransitionIn<AutoSend<Bot>>,
    ans: String,
) -> TransitionOut<Dialogue> {
    let num = state.number;

    if ans.starts_with("/get") {
        cx.answer(format!("Here is your number: {}", num)).await?;
        next(state)
    } else if ans.starts_with("/reset") {
        cx.answer("Resetted number").await?;
        next(StartState)
    } else {
        cx.answer("Please, send /get or /reset").await?;
        next(state)
    }
}
