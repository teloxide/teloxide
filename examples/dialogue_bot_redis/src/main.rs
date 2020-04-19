// This is a bot that asks your full name, your age, your favourite kind of
// music and sends all the gathered information back.
//
// # Example
// ```
//  - Let's start! First, what's your full name?
//  - Luke Skywalker
//  - What a wonderful name! Your age?
//  - 26
//  - Good. Now choose your favourite music
// *A keyboard of music kinds is displayed*
// *You select Metal*
//  - Metal
//  - Fine. Your full name: Luke Skywalker, your age: 26, your favourite music: Metal
// ```

#![allow(clippy::trivial_regex)]

#[macro_use]
extern crate smart_default;

use serde::{Deserialize, Serialize};
use std::sync::Arc;

use teloxide::{
    dispatching::dialogue::{RedisStorage, Serializer, Storage},
    prelude::*,
    types::{KeyboardButton, ReplyKeyboardMarkup},
};

use parse_display::{Display, FromStr};

// ============================================================================
// [Favourite music kinds]
// ============================================================================

#[derive(Copy, Clone, Display, FromStr)]
enum FavouriteMusic {
    Rock,
    Metal,
    Pop,
    Other,
}

impl FavouriteMusic {
    fn markup() -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::default().one_time_keyboard(true).append_row(vec![
            KeyboardButton::new("Rock"),
            KeyboardButton::new("Metal"),
            KeyboardButton::new("Pop"),
            KeyboardButton::new("Other"),
        ])
    }
}

// ============================================================================
// [A type-safe finite automaton]
// ============================================================================

#[derive(Clone, Serialize, Deserialize)]
struct ReceiveAgeState {
    full_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct ReceiveFavouriteMusicState {
    data: ReceiveAgeState,
    age: u8,
}

#[derive(Display)]
#[display(
    "Your full name: {data.data.full_name}, your age: {data.age}, your \
     favourite music: {favourite_music}"
)]
struct ExitState {
    data: ReceiveFavouriteMusicState,
    favourite_music: FavouriteMusic,
}

#[derive(SmartDefault, Serialize, Deserialize)]
enum Dialogue {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge(ReceiveAgeState),
    ReceiveFavouriteMusic(ReceiveFavouriteMusicState),
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Cx<State> = DialogueDispatcherHandlerCx<
    Message,
    State,
    <RedisStorage as Storage<Dialogue>>::Error,
>;
type Res = ResponseResult<DialogueStage<Dialogue>>;

async fn start(cx: Cx<()>) -> Res {
    cx.answer("Let's start! First, what's your full name?").send().await?;
    next(Dialogue::ReceiveFullName)
}

async fn full_name(cx: Cx<()>) -> Res {
    match cx.update.text() {
        None => {
            cx.answer("Please, send me a text message!").send().await?;
            next(Dialogue::ReceiveFullName)
        }
        Some(full_name) => {
            cx.answer("What a wonderful name! Your age?").send().await?;
            next(Dialogue::ReceiveAge(ReceiveAgeState {
                full_name: full_name.to_owned(),
            }))
        }
    }
}

async fn age(cx: Cx<ReceiveAgeState>) -> Res {
    match cx.update.text().unwrap().parse() {
        Ok(age) => {
            cx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(Dialogue::ReceiveFavouriteMusic(ReceiveFavouriteMusicState {
                data: cx.dialogue.unwrap(),
                age,
            }))
        }
        Err(_) => {
            cx.answer("Oh, please, enter a number!").send().await?;
            next(Dialogue::ReceiveAge(cx.dialogue.unwrap()))
        }
    }
}

async fn favourite_music(cx: Cx<ReceiveFavouriteMusicState>) -> Res {
    match cx.update.text().unwrap().parse() {
        Ok(favourite_music) => {
            cx.answer(format!(
                "Fine. {}",
                ExitState {
                    data: cx.dialogue.as_ref().unwrap().clone(),
                    favourite_music
                }
            ))
            .send()
            .await?;
            exit()
        }
        Err(_) => {
            cx.answer("Oh, please, enter from the keyboard!").send().await?;
            next(Dialogue::ReceiveFavouriteMusic(cx.dialogue.unwrap()))
        }
    }
}

async fn handle_message(cx: Cx<Dialogue>) -> Res {
    let DialogueDispatcherHandlerCx { bot, update, dialogue } = cx;
    match dialogue.unwrap() {
        Dialogue::Start => {
            start(DialogueDispatcherHandlerCx::new(bot, update, ())).await
        }
        Dialogue::ReceiveFullName => {
            full_name(DialogueDispatcherHandlerCx::new(bot, update, ())).await
        }
        Dialogue::ReceiveAge(s) => {
            age(DialogueDispatcherHandlerCx::new(bot, update, s)).await
        }
        Dialogue::ReceiveFavouriteMusic(s) => {
            favourite_music(DialogueDispatcherHandlerCx::new(bot, update, s))
                .await
        }
    }
}

// ============================================================================
// [Run!]
// ============================================================================

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            |cx| async move {
                handle_message(cx).await.expect("Something wrong with the bot!")
            },
            Arc::new(
                // You can also choose Serializer::JSON or Serializer::CBOR
                // All serializer but JSON require enabling feature
                // "serializer-<name>", e. g. "serializer-cbor"
                // or "serializer-bincode"
                RedisStorage::open("redis://127.0.0.1:6379", Serializer::Bincode)
                    .await
                    .unwrap(),
            ),
        ))
        .dispatch()
        .await;
}
