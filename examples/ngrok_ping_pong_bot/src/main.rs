// The version of ngrok ping-pong-bot, which uses a webhook to receive updates
// from Telegram, instead of long polling.

use teloxide::{dispatching::{update_listeners::{self, StatefulListener}, stop_token::AsyncStopToken}, prelude::*, types::Update};

use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;

use reqwest::StatusCode;

#[tokio::main]
async fn main() {
    run().await;
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook(bot: AutoSend<Bot>) -> impl update_listeners::UpdateListener<Infallible> {
    // You might want to specify a self-signed certificate via .certificate
    // method on SetWebhook.
    bot.set_webhook("Your HTTPS ngrok URL here. Get it by `ngrok http 80`")
        .await
        .expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::body::json())
        .map(move |json: serde_json::Value| {
            if let Ok(update) = Update::try_parse(&json) {
                tx.send(Ok(update)).expect("Cannot send an incoming update from the webhook")
            }

            StatusCode::OK
        })
        .recover(handle_rejection);

    let (stop_token, stop_flag) = AsyncStopToken::new_pair();

    let addr = "127.0.0.1:80".parse::<SocketAddr>().unwrap();
    let server = warp::serve(server);
    let (_addr, fut) = server.bind_with_graceful_shutdown(addr, stop_flag);

    // You might want to use serve.key_path/serve.cert_path methods here to
    // setup a self-signed TLS certificate.

    tokio::spawn(fut);
    let stream = UnboundedReceiverStream::new(rx);

    fn streamf<S, T>(state: &mut (S, T)) -> &mut S { &mut state.0 }
    
    StatefulListener::new((stream, stop_token), streamf, |state: &mut (_, AsyncStopToken)| state.1.clone(), None::<for<'a> fn(&'a _) -> _>)
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ngrok_ping_pong_bot...");

    let bot = Bot::from_env().auto_send();

    let cloned_bot = bot.clone();
    teloxide::repl_with_listener(
        bot,
        |message| async move {
            message.answer("pong").await?;
            respond(())
        },
        webhook(cloned_bot).await,
    )
    .await;
}
