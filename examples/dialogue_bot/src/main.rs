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
#![allow(dead_code)]

#[macro_use]
extern crate frunk;

use frunk::Coproduct;

use std::convert::Infallible;
use teloxide::{
    prelude::*,
    types::{KeyboardButton, ReplyKeyboardMarkup},
};

use parse_display::{Display, FromStr};
use teloxide::dispatching::dialogue::next;

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
        ReplyKeyboardMarkup::default().append_row(vec![
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

struct StartState;

struct ReceiveFullNameState {
    rest: StartState,
}

struct ReceiveAgeState {
    rest: ReceiveFullNameState,
    full_name: String,
}

struct ReceiveFavouriteMusicState {
    rest: ReceiveAgeState,
    age: u8,
}

#[derive(Display)]
#[display(
    "Your full name: {rest.rest.full_name}, your age: {rest.age}, your \
     favourite music: {favourite_music}"
)]
struct ExitState {
    rest: ReceiveFavouriteMusicState,
    favourite_music: FavouriteMusic,
}

up!(
    StartState -> ReceiveFullNameState,
    ReceiveFullNameState + [full_name: String] -> ReceiveAgeState,
    ReceiveAgeState + [age: u8] -> ReceiveFavouriteMusicState,
    ReceiveFavouriteMusicState + [favourite_music: FavouriteMusic] -> ExitState
);

type Dialogue = Coprod!(
    StartState,
    ReceiveFullNameState,
    ReceiveAgeState,
    ReceiveFavouriteMusicState
);

wrap_dialogue!(Wrapper, Dialogue);

impl Default for Wrapper {
    fn default() -> Self {
        Self(Dialogue::inject(StartState))
    }
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Cx<State> = DialogueDispatcherHandlerCx<Message, State, Infallible>;
type Res = ResponseResult<DialogueStage<Wrapper>>;

async fn start(cx: Cx<StartState>) -> Res {
    let DialogueDispatcherHandlerCx { cx, dialogue } = cx;
    let dialogue = dialogue.unwrap();

    cx.answer("Let's start! First, what's your full name?").send().await?;
    next(dialogue.up())
}

async fn full_name(cx: Cx<ReceiveFullNameState>) -> Res {
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

async fn age(cx: Cx<ReceiveAgeState>) -> Res {
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

async fn favourite_music(cx: Cx<ReceiveFavouriteMusicState>) -> Res {
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
        .messages_handler(DialogueDispatcher::new(|cx| async move {
            let DialogueDispatcherHandlerCx { cx, dialogue } = cx;

            // You need handle the error instead of panicking in real-world code, maybe
            // send diagnostics to a development chat.
            let Wrapper(dialogue) = dialogue.expect("Failed to get dialogue info from storage");

            dispatch!([cx, dialogue] -> [start, full_name, age, favourite_music])
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
