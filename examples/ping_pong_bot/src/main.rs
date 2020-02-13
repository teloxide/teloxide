use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env().enable_logging(crate_name!()).build();
    log::info!("Starting ping_pong_bot!");

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
