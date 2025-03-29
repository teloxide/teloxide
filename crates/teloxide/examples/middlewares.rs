use teloxide::prelude::*;

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting purchase bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        // Executes before the endpoint.
        .inspect(|msg: Message| println!("Before (message #{}).", msg.id))
        // Our "endpoint".
        .map_async(|bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "Inside the endpoint.").await?;
            Ok(())
        })
        // Executes after the endpoint.
        .inspect(|msg: Message| {
            println!("After (message #{}).", msg.id);
        })
        .endpoint(|result: HandlerResult| async move { result });

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}
