// This is a guess-a-number game!
//
// # Example
// ```
//  - Hello
//  - Let's play a game! Guess a number from 1 to 10 (inclusively).
//  - 4
//  - No.
//  - 3
//  - No.
//  - Blablabla
//  - Oh, please, send me a text message!
//  - 111
//  - Oh, please, send me a number in the range [1; 10]!
//  - 5
//  - Congratulations! You won!
// ```

#![allow(dead_code)]

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
    log::info!("Starting guess_a_number_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(|cx| async move {
            let DialogueDispatcherHandlerCx { cx, dialogue } = cx;

            // Unwrap without panic because of std::convert::Infallible.
            let Wrapper(dialogue) = dialogue.unwrap();

            dispatch!(
                [cx, dialogue] ->
                [start, receive_attempt]
            )
            .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
