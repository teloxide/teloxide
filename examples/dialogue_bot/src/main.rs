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

mod favourite_music;
mod states;
mod transitions;

use states::*;
use transitions::*;

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
            |cx| async move {
                let DialogueWithCx { cx, dialogue } = cx;

                // Unwrap without panic because of std::convert::Infallible.
                let dialogue = dialogue.unwrap();

                dispatch!(
                [cx, dialogue] ->
                [start, receive_full_name, receive_age, receive_favourite_music]
            )
            .expect("Something wrong with the bot!")
            },
            || Dialogue::inject(StartState),
        ))
        .dispatch()
        .await;
}
