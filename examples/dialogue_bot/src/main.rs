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

use std::convert::Infallible;
use teloxide::{
    prelude::*,
    types::{KeyboardButton, ReplyKeyboardMarkup},
};

use parse_display::{Display, FromStr};
use std::fmt::{Display, Error, Formatter};

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
// [Our finite automaton]
// ============================================================================

type FullName = String;
type Age = u8;

#[derive(Clone)]
struct ReceiveAgeState(FullName);

#[derive(Clone)]
struct ReceiveFavouriteMusicState(ReceiveAgeState, Age);

struct ExitState(ReceiveFavouriteMusicState, FavouriteMusic);

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge(ReceiveAgeState),
    ReceiveFavouriteMusic(ReceiveFavouriteMusicState),
}

impl Display for ExitState {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let ExitState(
            ReceiveFavouriteMusicState(ReceiveAgeState(full_name), age),
            favourite_music,
        ) = self;

        write!(
            f,
            "Your full name: {}, your age: {}, favourite music: {}",
            full_name, age, favourite_music
        )
    }
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Cx<State> = DialogueDispatcherHandlerCx<Message, State, Infallible>;
type Res = ResponseResult<DialogueStage<Dialogue>>;

async fn start(cx: Cx<()>) -> Res {
    req!(cx.answer("Let's start! First, what's your full name?"))?;
    next(Dialogue::ReceiveFullName)
}

async fn full_name(cx: Cx<()>) -> Res {
    match cx.update.text() {
        None => {
            req!(cx.answer("Please, send me a text message!"))?;
            next(Dialogue::ReceiveFullName)
        }
        Some(full_name) => {
            req!(cx.answer("What a wonderful name! Your age?"))?;
            next(Dialogue::ReceiveAge(ReceiveAgeState(full_name.to_owned())))
        }
    }
}

async fn age(cx: Cx<ReceiveAgeState>) -> Res {
    match cx.update.text().unwrap().parse() {
        Ok(age) => {
            req!(cx
                .answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup()))?;

            next(Dialogue::ReceiveFavouriteMusic(ReceiveFavouriteMusicState(
                cx.dialogue.unwrap(),
                age,
            )))
        }
        Err(_) => {
            req!(cx.answer("Oh, please, enter a number!"))?;
            next(Dialogue::ReceiveAge(cx.dialogue.unwrap()))
        }
    }
}

async fn favourite_music(cx: Cx<ReceiveFavouriteMusicState>) -> Res {
    match cx.update.text().unwrap().parse() {
        Ok(favourite_music) => {
            req!(cx.answer(format!(
                "Fine. {}",
                ExitState(cx.dialogue.clone().unwrap(), favourite_music)
            )))?;
            exit()
        }
        Err(_) => {
            req!(cx.answer("Oh, please, enter from the keyboard!"))?;
            next(Dialogue::ReceiveFavouriteMusic(cx.dialogue.unwrap()))
        }
    }
}

async fn handle_message(cx: Cx<Dialogue>) -> Res {
    let DialogueDispatcherHandlerCx { bot, update, dialogue } = cx;

    // You need handle the error instead of panicking in real-world code, maybe
    // send diagnostics to a development chat.
    match dialogue.expect("Failed to get dialogue info from storage") {
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
        .messages_handler(DialogueDispatcher::new(|cx| async move {
            handle_message(cx).await.expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
