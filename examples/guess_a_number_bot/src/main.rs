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
    ctx: DialogueHandlerCtx<Message, Dialogue>,
) -> Result<DialogueStage<Dialogue>, RequestError> {
    match ctx.dialogue {
        Dialogue::Start => {
            ctx.answer(
                "Let's play a game! Guess a number from 1 to 10 (inclusively).",
            )
            .send()
            .await?;
            next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
        }
        Dialogue::ReceiveAttempt(secret) => match ctx.update.text() {
            None => {
                ctx.answer("Oh, please, send me a text message!")
                    .send()
                    .await?;
                next(ctx.dialogue)
            }
            Some(text) => match text.parse::<u8>() {
                Ok(attempt) => match attempt {
                    x if !(1..=10).contains(&x) => {
                        ctx.answer(
                            "Oh, please, send me a number in the range [1; \
                             10]!",
                        )
                        .send()
                        .await?;
                        next(ctx.dialogue)
                    }
                    x if x == secret => {
                        ctx.answer("Congratulations! You won!").send().await?;
                        exit()
                    }
                    _ => {
                        ctx.answer("No.").send().await?;
                        next(ctx.dialogue)
                    }
                },
                Err(_) => {
                    ctx.answer(
                        "Oh, please, send me a number in the range [1; 10]!",
                    )
                    .send()
                    .await?;
                    next(ctx.dialogue)
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
        .message_handler(&DialogueDispatcher::new(|ctx| async move {
            handle_message(ctx)
                .await
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
