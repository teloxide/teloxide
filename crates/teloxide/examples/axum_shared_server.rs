// This example demonstrates how to embed teloxide's webhooks in an existing
// axum server.

use std::env;

use axum::{response::IntoResponse, routing::get};
use reqwest::StatusCode;
use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting axum_shared_server bot...");

    let bot = Bot::from_env();

    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([0, 0, 0, 0], port).into();

    let host = env::var("HOST").expect("HOST env variable is not set");
    let webhook_url = format!("https://{host}/webhook").parse().unwrap();

    let mut listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, webhook_url));

    let router = axum::Router::new()
        // Your routes...
        .route("/health", get(health))
        // Nested teloxide's axum router
        .nest("/", listener.take_router().unwrap());

    tokio::spawn(async move {
        axum::Server::bind(&addr)
            .serve(router.into_make_service())
            .await
            .expect("Axum server error")
    });

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            Ok(())
        },
        listener,
    )
    .await;

    // FIXME: with the current setup ^C does not work.
    //
    // This is because when `repl_with_listener` handles `^C`, it sends a stop
    // signal to the listener. But this signal is only actually processed in
    // `telegram_request` in `webhooks::Axum`'s internals. So, until a new
    // update is sent to the webhook, the updates stream created from the update
    // listener is waiting on a channel which is not closed, even though we
    // should have already stopped everything...
    //
    // This should be possible to fix by changing where we are handling the
    // signal.
}

async fn health() -> impl IntoResponse {
    StatusCode::OK
}
