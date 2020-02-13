use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    // Configure the fancy logger.
    std::env::set_var("RUST_LOG", "info");
    pretty_env_logger::init();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    // Create a dispatcher with a single message handler that answers "pong" to
    // each incoming message.
    Dispatcher::<RequestError>::new(bot)
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            ctx.answer("pong").send().await?;
            Ok(())
        })
        .dispatch()
        .await;
}
