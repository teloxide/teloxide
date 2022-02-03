// The version of ngrok ping-pong-bot, which uses a webhook to receive updates
// from Telegram, instead of long polling.

use teloxide::{
    dispatching::{
        stop_token::AsyncStopToken,
        update_listeners::{self, StatefulListener},
    },
    prelude2::*,
    types::Update,
};

use std::{convert::Infallible, net::SocketAddr};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::Filter;

use reqwest::{StatusCode, Url};

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting heroku_ping_pong_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::repl_with_listener(
        bot.clone(),
        |msg: Message, bot: AutoSend<Bot>| async move {
            bot.send_message(msg.chat.id, "pong").await?;
            respond(())
        },
        webhook(bot).await,
    )
    .await;
}

async fn handle_rejection(error: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    log::error!("Cannot process the request due to: {:?}", error);
    Ok(StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn webhook(bot: AutoSend<Bot>) -> impl update_listeners::UpdateListener<Infallible> {
    let url = Url::parse("Your HTTPS ngrok URL here. Get it by `ngrok http 80`").unwrap();

    // You might want to specify a self-signed certificate via .certificate
    // method on SetWebhook.
    bot.set_webhook(url).await.expect("Cannot setup a webhook");

    let (tx, rx) = mpsc::unbounded_channel();

    let server = warp::post()
        .and(warp::body::json())
        .map(move |update: Update| {
            tx.send(Ok(update)).expect("Cannot send an incoming update from the webhook");

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

    fn streamf<S, T>(state: &mut (S, T)) -> &mut S {
        &mut state.0
    }

    StatefulListener::new((stream, stop_token), streamf, |state: &mut (_, AsyncStopToken)| {
        state.1.clone()
    })
}
