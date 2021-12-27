// This bot answers how many messages it received in total on every message.

use std::sync::atomic::{AtomicU64, Ordering};

use lazy_static::lazy_static;
use teloxide::prelude::*;
use tokio_stream::wrappers::UnboundedReceiverStream;

lazy_static! {
    static ref MESSAGES_TOTAL: AtomicU64 = AtomicU64::new(0);
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting shared_state_bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |message| async move {
                let previous = MESSAGES_TOTAL.fetch_add(1, Ordering::Relaxed);

                message
                    .answer(format!("I received {} messages in total.", previous))
                    .await
                    .log_on_error()
                    .await;
            })
        })
        .dispatch()
        .await;
}
