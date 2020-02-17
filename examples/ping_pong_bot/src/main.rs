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
        .messages_handler(|messages: DispatcherHandlerRx<Message>| {
            messages.for_each_concurrent(None, |message| async move {
                message.answer("pong").send().await;
            })
        })
        .dispatch()
        .await;
}
