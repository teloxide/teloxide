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

type Cx<State> = DialogueDispatcherHandlerCx<Message, State>;
type Res = ResponseResult<DialogueStage<Dialogue>>;

async fn start(cx: Cx<()>) -> Res {
    cx.answer("Let's play a game! Guess a number from 1 to 10 (inclusively).")
        .send()
        .await?;
    next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
}

async fn receive_attempt(cx: Cx<u8>) -> Res {
    let secret = cx.dialogue;

    match cx.update.text() {
        None => {
            cx.answer("Oh, please, send me a text message!").send().await?;
            next(Dialogue::ReceiveAttempt(secret))
        }
        Some(text) => match text.parse::<u8>() {
            Ok(attempt) => {
                if attempt == secret {
                    cx.answer("Congratulations! You won!").send().await?;
                    exit()
                } else {
                    cx.answer("No.").send().await?;
                    next(Dialogue::ReceiveAttempt(secret))
                }
            }
            Err(_) => {
                cx.answer("Oh, please, send me a number in the range [1; 10]!")
                    .send()
                    .await?;
                next(Dialogue::ReceiveAttempt(secret))
            }
        },
    }
}

async fn handle_message(
    cx: DialogueDispatcherHandlerCx<Message, Dialogue>,
) -> Res {
    match cx {
        DialogueDispatcherHandlerCx {
            bot,
            update,
            dialogue: Dialogue::Start,
        } => start(DialogueDispatcherHandlerCx::new(bot, update, ())).await,
        DialogueDispatcherHandlerCx {
            bot,
            update,
            dialogue: Dialogue::ReceiveAttempt(secret),
        } => {
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
