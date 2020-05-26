use teloxide::prelude::*;

use super::{favourite_music::FavouriteMusic, states::*};

pub type In<State> = TransitionIn<State>;
pub type Out = TransitionOut<Wrapper>;

pub async fn start(cx: In<StartState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    cx.answer("Let's start! First, what's your full name?").send().await?;
    next(dialogue.up())
}

pub async fn receive_full_name(cx: In<ReceiveFullNameState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text_owned() {
        Some(full_name) => {
            cx.answer("What a wonderful name! Your age?").send().await?;
            next(dialogue.up(full_name))
        }
        None => {
            cx.answer("Please, enter a text message!").send().await?;
            next(dialogue)
        }
    }
}

pub async fn receive_age(cx: In<ReceiveAgeState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text().map(str::parse) {
        Some(Ok(age)) => {
            cx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(dialogue.up(age))
        }
        _ => {
            cx.answer("Please, enter a number!").send().await?;
            next(dialogue)
        }
    }
}

pub async fn receive_favourite_music(
    cx: In<ReceiveFavouriteMusicState>,
) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text().map(str::parse) {
        Some(Ok(favourite_music)) => {
            cx.answer(format!("Fine. {}", dialogue.up(favourite_music)))
                .send()
                .await?;
            exit()
        }
        _ => {
            cx.answer("Please, enter from the keyboard!").send().await?;
            next(dialogue)
        }
    }
}
