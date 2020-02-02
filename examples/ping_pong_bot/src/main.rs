use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    Dispatcher::new(Bot::new("MyAwesomeToken"))
        .message_handler(|ctx: HandlerCtx<Message>| async move {
            ctx.reply("pong").await
        })
        .dispatch()
        .await;
}
