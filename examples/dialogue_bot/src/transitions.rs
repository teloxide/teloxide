use teloxide::prelude::*;

use super::{favourite_music::FavouriteMusic, states::*};

pub type Cx = UpdateWithCx<Message>;
pub type Out = TransitionOut<Dialogue>;

async fn start(cx: Cx, state: StartState) -> Out {
    cx.answer_str("Let's start! First, what's your full name?").await?;
    next(state.up())
}

async fn receive_full_name(cx: Cx, state: ReceiveFullNameState) -> Out {
    match cx.update.text_owned() {
        Some(full_name) => {
            cx.answer_str("What a wonderful name! Your age?").await?;
            next(state.up(full_name))
        }
        _ => {
            cx.answer_str("Please, enter a text message!").await?;
            next(state)
        }
    }
}

async fn receive_age(cx: Cx, state: ReceiveAgeState) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(age)) => {
            cx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(state.up(age))
        }
        _ => {
            cx.answer_str("Please, enter a number!").await?;
            next(state)
        }
    }
}

async fn receive_favourite_music(
    cx: Cx,
    state: ReceiveFavouriteMusicState,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(favourite_music)) => {
            cx.answer_str(format!("Fine. {}", state.up(favourite_music)))
                .await?;
            exit()
        }
        _ => {
            cx.answer_str("Please, enter from the keyboard!").await?;
            next(state)
        }
    }
}

pub async fn dispatch(cx: Cx, dialogue: Dialogue) -> Out {
    match dialogue {
        Dialogue::Start(state) => start(cx, state).await,
        Dialogue::ReceiveFullName(state) => receive_full_name(cx, state).await,
        Dialogue::ReceiveAge(state) => receive_age(cx, state).await,
        Dialogue::ReceiveFavouriteMusic(state) => {
            receive_favourite_music(cx, state).await
        }
    }
}
