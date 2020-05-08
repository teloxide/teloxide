// This bot just answers "pong" to each incoming UpdateKind::Message.

use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                req!(message.answer("pong")).log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
