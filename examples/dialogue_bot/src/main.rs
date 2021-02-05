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
extern crate frunk;

mod dialogue;

use crate::dialogue::{MyDialogue};
use teloxide::prelude::*;
use std::convert::Infallible;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env();

    teloxide::dialogues_repl(bot, |DialogueWithCx { cx, dialogue }: _| async move {
        let dialogue = dialogue.unwrap();
        handle_message(cx, dialogue).await.expect("Something wrong with the bot!")
    })
    .await;
}

async fn handle_message(cx: UpdateWithCx<Message>, dialogue: Dialogue<MyDialogue, Infallible>) -> TransitionOut {
    match cx.update.text_owned() {
        None => {
            cx.answer_str("Send me a text message.").await?;
            dialogue.stay().await;

            Ok(())
        }
        Some(ans) => Transition::react(dialogue, cx, ans).await,
    }
}
