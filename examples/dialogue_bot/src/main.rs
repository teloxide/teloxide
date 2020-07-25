// This is a bot that asks you three questions, e.g. a simple test.
//
// # Example
// ```
//  - Let's start our test! How many days per week are there?
//  - 7
//  - 10*5 = ?
//  - 50
//  - What's an alternative name of Gandalf?
//  - Mithrandir
//  - Congratulations! You've successfully passed the test!
// ```

#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate frunk;

mod states;
mod transitions;

use states::*;

use std::convert::Infallible;
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |input: DialogueWithCx<Message, Dialogue, Infallible>| async move {
                // Unwrap without panic because of std::convert::Infallible.
                input
                    .dialogue
                    .unwrap()
                    .react(input.cx)
                    .await
                    .expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}
