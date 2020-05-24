use teloxide::prelude::*;

use super::{favourite_music::FavouriteMusic, states::*};

pub type Cx<State> =
    DialogueDispatcherHandlerCx<Message, State, std::convert::Infallible>;
pub type Res = ResponseResult<DialogueStage<Wrapper>>;

pub async fn start(cx: Cx<StartState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    cx.answer("Let's start! First, what's your full name?").send().await?;
    next(dialogue.up())
}

pub async fn receive_full_name(cx: Cx<ReceiveFullNameState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    match cx.update.text_owned() {
        Some(full_name) => {
            cx.answer("What a wonderful name! Your age?").send().await?;
            next(dialogue.up(full_name))
        }
        None => {
            cx.answer("Please, send me a text message!").send().await?;
            next(dialogue)
        }
    }
}

pub async fn receive_age(cx: Cx<ReceiveAgeState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    match cx.update.text().unwrap().parse() {
        Ok(age) => {
            cx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(dialogue.up(age))
        }
        Err(_) => {
            cx.answer("Oh, please, enter a number!").send().await?;
            next(dialogue)
        }
    }
}

pub async fn receive_favourite_music(
    cx: Cx<ReceiveFavouriteMusicState>,
) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    match cx.update.text().unwrap().parse() {
        Ok(favourite_music) => {
            cx.answer(format!("Fine. {}", dialogue.up(favourite_music)))
                .send()
                .await?;
            exit()
        }
        Err(_) => {
            cx.answer("Oh, please, enter from the keyboard!").send().await?;
            next(dialogue)
        }
    }
}
