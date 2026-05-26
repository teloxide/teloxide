// Demonstrates the update_stream API — a simpler alternative to
// Dispatcher/dptree. Echoes any text message back to the sender.

use futures::StreamExt;
use teloxide::{prelude::*, types::UpdateKind};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting update_stream echo bot...");

    let bot = Bot::from_env();
    let token = CancellationToken::new();

    let cancel = token.clone();
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.ok();
        log::info!("Shutting down...");
        cancel.cancel();
    });

    let mut stream =
        bot.update_stream().token(token).build().await.expect("Failed to start update stream");

    while let Some(result) = stream.next().await {
        let update = match result {
            Ok(update) => update,
            Err(e) => {
                log::error!("Error receiving update: {e}");
                continue;
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
