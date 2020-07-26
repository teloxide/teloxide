// This is a bot that asks you three questions, e.g. a simple test.
//
// # Example
// ```
//  - Hey
//  - Let's start! What's your full name?
//  - Gandalf the Grey
//  - How old are you?
//  - 223
//  - What's your location?
//  - Middle-earth
//  - Full name: Gandalf the Grey
//    Age: 223
//    Location: Middle-earth
// ```

#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate frunk;
extern crate frunk_core;
#[macro_use]
extern crate teloxide_macros;

mod dialogue;

use crate::dialogue::Dialogue;
use std::convert::Infallible;
use teloxide::prelude::*;

type In = DialogueWithCx<Message, Dialogue, Infallible>;

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
            |DialogueWithCx { cx, dialogue }: In| async move {
                // No panic because of std::convert::Infallible.
                let dialogue = dialogue.unwrap();
                handle_message(cx, dialogue)
                    .await
                    .expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}

async fn handle_message(
    cx: UpdateWithCx<Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text_owned() {
        None => {
            cx.answer_str("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}
