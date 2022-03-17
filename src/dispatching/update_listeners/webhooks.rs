use std::{convert::Infallible, net::SocketAddr};

use crate::{dispatching::update_listeners::UpdateListener, requests::Requester, types::InputFile};

/// Options related to setting up webhooks.
pub struct Options {
    /// Local address to listen to.
    pub address: SocketAddr,

    /// Public url that Telegram will send updates to.
    ///
    /// Note:
    /// - At the time of writing only ports 443, 80, 88 and 8443 [are
    ///   supported][set_webhook]
    /// - This url must be forwarded to the [address][addr] in order for webhook
    ///   to work
    /// - This url should be kept private, otherwise malicious actors can
    ///   pretend to be Telegram and send fake updates to your bot
    ///
    /// [set_webhook]: https://core.telegram.org/bots/api#setwebhook
    /// [addr]: (self::Options.address)
    pub url: url::Url,

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    ///
    /// Default - None.
    pub certificate: Option<InputFile>,

    /// Pass `true` to drop all pending updates.
    ///
    /// Default - None.
    pub drop_pending_updates: Option<bool>,
}

impl Options {
    /// Construct a new webhook options, see [`Options.address`] and
    /// [`Options.url`] for details.
    pub fn new(address: SocketAddr, url: url::Url) -> Self {
        Self { address, url, certificate: None, drop_pending_updates: None }
    }

    /// Upload your public key certificate so that the root certificate in use
    /// can be checked. See Telegram's [self-signed guide] for details.
    ///
    /// [self-signed guide]: https://core.telegram.org/bots/self-signed
    pub fn certificate(self, v: InputFile) -> Self {
        Self { certificate: Some(v), ..self }
    }

    /// Drop all pending updates before setting up webhook.
    pub fn drop_pending_updates(self) -> Self {
        Self { drop_pending_updates: Some(true), ..self }
    }
}

/// Webhook implementation based on the [axum] framework.
///
/// ## Panics
///
/// If binding to the [address] fails.
///
/// [address]: Options.address
///
/// ## Errors
///
/// If `set_webhook()` fails.
#[cfg(feature = "webhooks-axum")]
pub async fn axum<R>(bot: R, options: Options) -> Result<impl UpdateListener<Infallible>, R::Err>
where
    R: Requester + Send + 'static,
    <R as Requester>::DeleteWebhook: Send,
{
    use crate::{
        dispatching::{stop_token::AsyncStopToken, update_listeners},
        requests::Request,
        types::Update,
    };
    use axum::{
        extract::Extension, http::StatusCode, response::IntoResponse, routing::post,
        AddExtensionLayer,
    };
    use futures::FutureExt;
    use teloxide_core::requests::HasPayload;
    use tokio::sync::mpsc;
    use tokio_stream::wrappers::UnboundedReceiverStream;
    use tower::ServiceBuilder;
    use tower_http::trace::TraceLayer;

    type Sender = mpsc::UnboundedSender<Result<Update, std::convert::Infallible>>;

    let Options { address, url, certificate, drop_pending_updates } = options;

    {
        let mut req = bot.set_webhook(url.clone());
        req.payload_mut().certificate = certificate;
        req.payload_mut().drop_pending_updates = drop_pending_updates;

        req.send().await?;
    }

    let (tx, rx): (Sender, _) = mpsc::unbounded_channel();

    async fn telegram_request(input: String, tx: Extension<Sender>) -> impl IntoResponse {
        match serde_json::from_str(&input) {
            Ok(update) => {
                tx.send(Ok(update)).expect("Cannot send an incoming update from the webhook")
            }
            Err(error) => {
                log::error!(
                    "Cannot parse an update.\nError: {:?}\nValue: {}\n\
                     This is a bug in teloxide-core, please open an issue here: \
                     https://github.com/teloxide/teloxide/issues.",
                    error,
                    input
                );
            }
        };

        StatusCode::OK
    }

    let app = axum::Router::new().route(url.path(), post(telegram_request)).layer(
        ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(AddExtensionLayer::new(tx))
            .into_inner(),
    );

    let (stop_token, stop_flag) = AsyncStopToken::new_pair();

    tokio::spawn(async move {
        axum::Server::bind(&address)
            .serve(app.into_make_service())
            .with_graceful_shutdown(stop_flag.then(move |()| async move {
                // This assignment is needed to not require `R: Sync` since without it `&bot`
                // temporary lives across `.await` points.
                let req = bot.delete_webhook().send();
                let res = req.await;
                if let Err(err) = res {
                    log::error!("Couldn't delete webhook: {}", err);
                }
            }))
            .await
            .expect("Axum server error")
    });

    let stream = UnboundedReceiverStream::new(rx);

    fn streamf<S, T>(state: &mut (S, T)) -> &mut S {
        &mut state.0
    }

    let listener = update_listeners::StatefulListener::new(
        (stream, stop_token),
        streamf,
        |state: &mut (_, AsyncStopToken)| state.1.clone(),
    );

    Ok(listener)
}
