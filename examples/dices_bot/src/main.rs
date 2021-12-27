// This bot throws a dice on each incoming message.

use std::sync::Arc;
use teloxide::prelude::*;

type TeleBot = AutoSend<Bot>;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Arc<Message>, bot: Arc<TeleBot>| async move {
        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}
