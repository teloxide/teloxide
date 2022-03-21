// This bot throws a dice on each incoming message.

use teloxide::prelude2::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}
