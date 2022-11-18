// The version of Heroku ping-pong-bot, which uses a webhook to receive updates
// from Telegram, instead of long polling.
//
// You will need to configure the buildpack for heroku. We will be using Heroku
// rust buildpack [1]. Configuration was done by using heroku CLI.
//
// If you're creating a new Heroku application, run this:
//
// ```
// heroku create --buildpack emk/rust
// ```
//
// To set buildpack for existing application:
//
// ```
// heroku buildpacks:set emk/rust
// ```
//
// [1]: https://github.com/emk/heroku-buildpack-rust

use std::env;

use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting Heroku ping-pong bot...");

    let bot = Bot::from_env();

    // Heroku auto defines a port value
    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([0, 0, 0, 0], port).into();

    // Heroku host example: "heroku-ping-pong-bot.herokuapp.com"
    let host = env::var("HOST").expect("HOST env variable is not set");
    let url = format!("https://{host}/webhook").parse().unwrap();

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
    .await;
}
