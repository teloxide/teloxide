use teloxide::prelude::*;

use std::env::{set_var, var};

#[tokio::main]
async fn main() {
    // Configure a fancy logger. Let this bot print everything, but restrict
    // teloxide to only log errors.
    set_var("RUST_LOG", "ping_pong_bot=trace");
    set_var("RUST_LOG", "teloxide=error");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    let bot = Bot::new(var("TELOXIDE_TOKEN").unwrap());

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
