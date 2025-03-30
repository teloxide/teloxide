use teloxide::prelude::*;

type HandlerResult = Result<(), teloxide::RequestError>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting middleware bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        // Executes before the endpoint.
        .inspect(|msg: Message| println!("Before (message #{}).", msg.id))
        // Our "endpoint".
        .map_async(my_endpoint)
        // Executes after the endpoint (even if it fails).
        .inspect(|msg: Message| {
            println!("After (message #{}).", msg.id);
        })
        // Retrieve the result of the endpoint and pass it to the dispatcher.
        .endpoint(|result: HandlerResult| async move {
            // Alternatively, we could also pattern-match on this value for more
            // fine-grained behaviour.
            result
        });

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}

async fn my_endpoint(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Inside the endpoint.").await?;
    Ok(())
}
