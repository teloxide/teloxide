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

#[macro_use]
extern crate smart_default;

use teloxide::prelude::*;

use rand::{thread_rng, Rng};
use std::convert::Infallible;

// ============================================================================
// [Our finite automaton]
// ============================================================================

type Secret = u8;

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveAttempt(Secret),
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

type Cx<State> = DialogueDispatcherHandlerCx<Message, State, Infallible>;
type Res = ResponseResult<DialogueStage<Dialogue>>;

async fn start(cx: Cx<()>) -> Res {
    req!(cx.answer(
        "Let's play a game! Guess a number from 1 to 10 (inclusively)."
    ))?;
    next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
}

async fn receive_attempt(cx: Cx<Secret>) -> Res {
    let secret = cx.dialogue.unwrap();

    match cx.update.text() {
        None => {
            req!(cx.answer("Oh, please, send me a text message!"))?;
            next(Dialogue::ReceiveAttempt(secret))
        }
        Some(text) => match text.parse::<Secret>() {
            Ok(attempt) => {
                if attempt == secret {
                    req!(cx.answer("Congratulations! You won!"))?;
                    exit()
                } else {
                    req!(cx.answer("No."))?;
                    next(Dialogue::ReceiveAttempt(secret))
                }
            }
            Err(_) => {
                req!(cx.answer(
                    "Oh, please, send me a number in the range [1; 10]!"
                ))?;
                next(Dialogue::ReceiveAttempt(secret))
            }
        },
    }
}

async fn handle_message(
    cx: DialogueDispatcherHandlerCx<Message, Dialogue, Infallible>,
) -> Res {
    let DialogueDispatcherHandlerCx { bot, update, dialogue } = cx;

    // You need handle the error instead of panicking in real-world code, maybe
    // send diagnostics to a development chat.
    match dialogue.expect("Failed to get dialogue info from storage") {
        Dialogue::Start => {
            start(DialogueDispatcherHandlerCx::new(bot, update, ())).await
        }
        Dialogue::ReceiveAttempt(secret) => {
            receive_attempt(DialogueDispatcherHandlerCx::new(
                bot, update, secret,
            ))
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
    log::info!("Starting guess_a_number_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(|cx| async move {
            handle_message(cx).await.expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
