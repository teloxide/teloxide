use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "ping_pong_bot=trace");
    pretty_env_logger::init();
    log::info!("Starting the ping-pong bot!");

    let bot = Bot::new("1061598315:AAErEDodTsrqD3UxA_EvFyEfXbKA6DT25G0");

    Dispatcher::new(bot)
        .message_handler(|ctx: HandlerCtx<Message>| ctx.reply("pong"))
        .dispatch()
        .await;
}
