// This bot just answers "pong" to each incoming UpdateKind::Message.

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    polling_default(bot)
        .basic_config()
        .for_each_concurrent(None, |message| async move {
            message.answer("pong").send().await.log_on_error().await;
        })
        .await;
}
