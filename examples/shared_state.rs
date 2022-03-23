// This bot answers how many messages it received in total on every message.

use std::sync::atomic::{AtomicU64, Ordering};

use once_cell::sync::Lazy;
use teloxide::prelude2::*;

static MESSAGES_TOTAL: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
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
