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
extern crate smart_default;
#[macro_use]
extern crate derive_more;

mod favourite_music;
mod states;
mod transitions;

use states::*;
use transitions::*;

use std::convert::Infallible;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |input: TransitionIn<Dialogue, Infallible>| async move {
                // Unwrap without panic because of std::convert::Infallible.
                dispatch(input.cx, input.dialogue.unwrap())
                    .await
                    .expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}
