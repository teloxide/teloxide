use teloxide::{
    prelude::*,
    types::{
        InlineQueryResult, InlineQueryResultArticle, InputMessageContent, InputMessageContentText,
    },
    Bot,
};
use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env().auto_send();
    // Create a new dispatcher to handle incoming queries
    Dispatcher::new(bot)
        .inline_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, InlineQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |query| async move {
                // First, create your actual response
                let google_search = InlineQueryResultArticle::new(
                    // Each item needs a unique ID, as well as the response container for the
                    // items. These can be whatever, as long as they don't
                    // conflict.
                    "01".to_string(),
                    // What the user will actually see
                    "Google Search",
                    // What message will be sent when clicked/tapped
                    InputMessageContent::Text(InputMessageContentText::new(format!(
                        "https://www.google.com/search?q={}",
                        query.update.query,
                    ))),
                );
                // While constructing them from the struct itself is possible, it is preferred
                // to use the builder pattern if you wish to add more
                // information to your result. Please refer to the documentation
                // for more detailed information about each field. https://docs.rs/teloxide/0.5.1/teloxide/types/struct.InlineQueryResultArticle.html
                let ddg_search = InlineQueryResultArticle::new(
                    "02".to_string(),
                    "DuckDuckGo Search".to_string(),
                    InputMessageContent::Text(InputMessageContentText::new(format!(
                        "https://duckduckgo.com/?q={}",
                        query.update.query.to_string()
                    ))),
                )
                .description("DuckDuckGo Search")
                .thumb_url("https://duckduckgo.com/assets/logo_header.v108.png")
                .url("https://duckduckgo.com/about"); // Note: This is the url that will open if they click the thumbnail

                let results = vec![
                    InlineQueryResult::Article(google_search),
                    InlineQueryResult::Article(ddg_search),
                ];

                // Send it off! One thing to note -- the ID we use here must be of the query
                // we're responding to.
                let response =
                    query.requester.answer_inline_query(&query.update.id, results).send().await;
                if let Err(err) = response {
                    log::error!("Error in handler: {:?}", err);
                }
            })
        })
        .dispatch()
        .await;
}
