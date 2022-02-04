// This bot answers how many messages it received in total on every message.

use std::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use teloxide::prelude2::*;

lazy_static! {
    static ref MESSAGES_TOTAL: AtomicU64 = AtomicU64::new(0);
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting shared_state_bot...");

    let bot = Bot::from_env().auto_send();

    let handler = Update::filter_message().branch(dptree::endpoint(
        |msg: Message, bot: AutoSend<Bot>| async move {
            let previous = MESSAGES_TOTAL.fetch_add(1, Ordering::Relaxed);
            bot.send_message(msg.chat.id, format!("I received {} messages in total.", previous))
                .await?;
            respond(())
        },
    ));

    Dispatcher::builder(bot, handler).build().setup_ctrlc_handler().dispatch().await;
}
