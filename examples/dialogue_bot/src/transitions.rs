use teloxide::prelude::*;

use super::{favourite_music::FavouriteMusic, states::*};

pub type In<State> = TransitionIn<State, std::convert::Infallible>;
pub type Out = TransitionOut<Dialogue>;

pub async fn start(cx: In<StartState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    cx.answer_str("Let's start! First, what's your full name?").await?;
    next(dialogue.up())
}

pub async fn receive_full_name(cx: In<ReceiveFullNameState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text_owned() {
        Some(full_name) => {
            cx.answer_str("What a wonderful name! Your age?").await?;
            next(dialogue.up(full_name))
        }
        _ => {
            cx.answer_str("Please, enter a text message!").await?;
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
            cx.answer_str("Please, enter a number!").await?;
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
            cx.answer_str(format!("Fine. {}", dialogue.up(favourite_music)))
                .await?;
            exit()
        }
        _ => {
            cx.answer_str("Please, enter from the keyboard!").await?;
            next(dialogue)
        }
    }
}
