// This bot throws a dice on each incoming message.

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |message: Message, bot: Bot| async move {
        bot.send_dice(message.chat.id).await?;
        Ok(())
    })
    .await;
}
