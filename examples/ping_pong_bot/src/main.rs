use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    // Configure a fancy logger. Let this bot print everything, but restrict
    // teloxide to only log errors.
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    std::env::set_var("RUST_LOG", "teloxide=error");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    // Creates a dispatcher of updates with the specified bot. Don't forget to
    // replace `MyAwesomeToken` with yours.
    Dispatcher::<RequestError>::new(Bot::new("MyAwesomeToken"))
        // Registers a message handler. Inside a body of the closure, answer
        // `"pong"` to an incoming message.
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            ctx.answer("pong").send().await?;
            Ok(())
        })
        .dispatch()
        .await;
}
