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

// ============================================================================
// [A type-safe finite automaton]
// ============================================================================

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveAttempt(u8),
}

// ============================================================================
// [Control a dialogue]
// ============================================================================

async fn handle_message(
    cx: DialogueDispatcherHandlerCx<Message, Dialogue>,
) -> ResponseResult<DialogueStage<Dialogue>> {
    match cx.dialogue {
        Dialogue::Start => {
            cx.answer(
                "Let's play a game! Guess a number from 1 to 10 (inclusively).",
            )
            .send()
            .await?;
            next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
        }
        Dialogue::ReceiveAttempt(secret) => match cx.update.text() {
            None => {
                cx.answer("Oh, please, send me a text message!").send().await?;
                next(cx.dialogue)
            }
            Some(text) => match text.parse::<u8>() {
                Ok(attempt) => match attempt {
                    x if !(1..=10).contains(&x) => {
                        cx.answer(
                            "Oh, please, send me a number in the range [1; \
                             10]!",
                        )
                        .send()
                        .await?;
                        next(cx.dialogue)
                    }
                    x if x == secret => {
                        cx.answer("Congratulations! You won!").send().await?;
                        exit()
                    }
                    _ => {
                        cx.answer("No.").send().await?;
                        next(cx.dialogue)
                    }
                },
                Err(_) => {
                    cx.answer(
                        "Oh, please, send me a number in the range [1; 10]!",
                    )
                    .send()
                    .await?;
                    next(cx.dialogue)
                }
            },
        },
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
