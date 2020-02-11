use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    std::env::set_var("RUST_LOG", "teloxide=error");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    Dispatcher::<RequestError>::new(Bot::new("MyAwesomeToken"))
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            ctx.answer("pong").send().await?;
            Ok(())
        })
        .dispatch()
        .await;
}
