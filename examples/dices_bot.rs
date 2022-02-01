// This bot throws a dice on each incoming message.

use teloxide::prelude2::*;

type TeleBot = AutoSend<Bot>;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: TeleBot| async move {
        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}
