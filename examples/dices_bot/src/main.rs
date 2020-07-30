// This bot throws a dice on each incoming message.

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    repl(bot, |message| async move {
        message.send_dice().send().await?;
        Ok(())
    })
    .await;
}
