// Demonstrates the update_stream API, a simpler alternative to
// Dispatcher/dptree. Echoes any text message back to the sender.

use futures::StreamExt;
use teloxide::{prelude::*, types::UpdateKind};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting update_stream echo bot...");

    let bot = Bot::from_env();

    let mut stream = bot
        .update_stream()
        .build()
        .await
        .expect("Failed to start update stream");

    loop {
        let update = tokio::select! {
            item = stream.next() => match item {
                Some(Ok(update)) => update,
                Some(Err(e)) => {
                    log::error!("Error receiving update: {e}");
                    continue;
                }
                None => break,
            },
            _ = tokio::signal::ctrl_c() => {
                log::info!("Shutting down...");
                stream.shutdown();
                break;
            }
        };

        if let UpdateKind::Message(msg) = update.kind {
            if let Some(text) = msg.text() {
                let bot = bot.clone();
                let chat_id = msg.chat.id;
                let text = text.to_owned();
                tokio::spawn(async move {
                    if let Err(e) = bot.send_message(chat_id, text).await {
                        log::error!("Failed to send message: {e}");
                    }
                });
            }
        }
    }

    log::info!("Bot stopped.");
}
