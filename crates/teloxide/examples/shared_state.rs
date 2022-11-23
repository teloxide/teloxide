// This bot answers how many messages it received in total on every message.

use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc,
};

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting shared state bot...");

    let bot = Bot::from_env();
    let messages_total = Arc::new(AtomicU64::new(0));

    let handler = Update::filter_message().endpoint(
        |bot: Bot, messages_total: Arc<AtomicU64>, msg: Message| async move {
            let previous = messages_total.fetch_add(1, Ordering::Relaxed);
            bot.send_message(msg.chat.id, format!("I received {previous} messages in total."))
                .await?;
            respond(())
        },
    );

    Dispatcher::builder(bot, handler)
        // Pass the shared state to the handler as a dependency.
        .dependencies(dptree::deps![messages_total])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
