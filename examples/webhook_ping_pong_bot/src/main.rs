// The version of ping-pong-bot, which uses a webhook to receive updates from
// Telegram, instead of long polling.

use teloxide::{dispatching::update_listeners, prelude::*};

use std::{convert::Infallible, net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;
use warp::Filter;

use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_rejection(
    error: warp::Rejection,
) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook<'a>(
    bot: Arc<Bot>,
) -> impl update_listeners::UpdateListener<Infallible> {
    // You might want to specify a self-signed certificate via .certificate
    // method on SetWebhook.
    bot.set_webhook("Your HTTPS ngrok URL here. Get it by 'ngrok http 80'")
        .send()
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            match serde_json::from_str::<Update>(&json.to_string()) {
                Ok(update) => tx
                    .send(Ok(update))
                    .expect("Cannot send an incoming update from the webhook"),
                Err(error) => {
                    // In this case, please report a bug at https://github.com/teloxide/teloxide/issues !!!
                    log::error!(
                        "Cannot parse Update: {}\nError: {}",
                        json,
                        error
                    );
                }
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let serve = warp::serve(server);

    // You might want to use serve.key_path/serve.cert_path methods here to
    // setup a self-signed TLS certificate.

    tokio::spawn(serve.run("127.0.0.1:80".parse::<SocketAddr>().unwrap()));
    rx
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(Arc::clone(&bot))
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.answer("pong").send().await.log_on_error().await;
            })
        })
        .dispatch_with_listener(
            webhook(bot).await,
            LoggingErrorHandler::with_custom_text(
                "An error from the update listener",
            ),
        )
        .await;
}
