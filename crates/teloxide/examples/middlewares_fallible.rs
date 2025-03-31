use teloxide::prelude::*;

type HandlerResult = Result<(), teloxide::RequestError>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting middlewares_fallible bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message()
        // Executes before the endpoint.
        .inspect(|msg: Message| println!("Before (message #{}).", msg.id))
        // Our "endpoint".
        .map_async(my_endpoint)
        // Executes after the endpoint. If the endpoint failed, print an error message and stop
        // execution; if not, continue.
        .filter(|result: HandlerResult, msg: Message| {
            println!("In-between (message #{}).", msg.id);
            match result {
                Ok(()) => true,
                Err(err) => {
                    eprintln!("Our endpoint failed: {err}");
                    false
                }
            }
        })
        // Executes only if the endpoint succeeded.
        .endpoint(|msg: Message| async move {
            println!("After (message #{}).", msg.id);
            HandlerResult::Ok(())
        });

    Dispatcher::builder(bot, handler).enable_ctrlc_handler().build().dispatch().await;
}

async fn my_endpoint(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Inside the endpoint.").await?;
    Ok(())
}
