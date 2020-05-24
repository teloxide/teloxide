use rand::{thread_rng, Rng};
use teloxide::prelude::*;

use super::states::*;

pub type Cx<State> =
    DialogueDispatcherHandlerCx<Message, State, std::convert::Infallible>;
pub type Res = ResponseResult<DialogueStage<Wrapper>>;

pub async fn start(cx: Cx<StartState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    cx.answer("Let's play a game! Guess a number from 1 to 10 (inclusively).")
        .send()
        .await?;

    next(dialogue.up(thread_rng().gen_range(1, 11)))
}

pub async fn receive_attempt(cx: Cx<ReceiveAttemptState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    match cx.update.text() {
        None => {
            cx.answer("Oh, please, send me a text message!").send().await?;
            next(dialogue)
        }
        Some(text) => match text.parse::<u8>() {
            Ok(attempt) => {
                if attempt == dialogue.secret {
                    cx.answer("Congratulations! You won!").send().await?;
                    exit()
                } else {
                    cx.answer("No.").send().await?;
                    next(dialogue)
                }
            }
            Err(_) => {
                cx.answer("Oh, please, send me a number in the range [1; 10]!")
                    .send()
                    .await?;
                next(dialogue)
            }
        },
    }
}
