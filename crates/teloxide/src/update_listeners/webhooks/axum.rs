use std::{convert::Infallible, future::Future};

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, status::StatusCode},
};
use tokio::sync::mpsc;

use crate::{
    requests::Requester,
    stop::StopFlag,
    types::{Update, UpdateKind},
    update_listeners::{webhooks::Options, UpdateListener},
};

/// Webhook implementation based on the [mod@axum] framework.
///
/// This function does all the work necessary for webhook to work, it:
/// - Calls [`set_webhook`], so telegram starts sending updates our way
/// - Spawns [mod@axum] server listening for updates
/// - When the update listener is [`stop`]ped, calls [`delete_webhook`]
///
/// [`set_webhook`]: crate::payloads::SetWebhook
/// [`delete_webhook`]: crate::payloads::DeleteWebhook
/// [`stop`]: crate::stop::StopToken::stop
///
/// ## Panics
///
/// If binding to the [address] fails.
///
/// [address]: Options::address
///
/// ## Fails
///
/// If `set_webhook()` fails.
///
/// ## See also
///
/// [`axum_to_router`] and [`axum_no_setup`] for lower-level versions of this
/// function.
pub async fn axum<R>(
    bot: R,
    options: Options,
) -> Result<impl UpdateListener<Err = Infallible>, R::Err>
where
    R: Requester + Send + 'static,
    <R as Requester>::DeleteWebhook: Send,
{
    let Options { address, .. } = options;

    let (mut update_listener, stop_flag, app) = axum_to_router(bot, options).await?;
    let stop_token = update_listener.stop_token();

    tokio::spawn(async move {
        let tcp_listener = tokio::net::TcpListener::bind(address)
            .await
            .inspect_err(|_| stop_token.stop())
            .expect("Couldn't bind to the address");
        axum::serve(tcp_listener, app)
            .with_graceful_shutdown(stop_flag)
            .await
            .inspect_err(|_| stop_token.stop())
            .expect("Axum server error");
    });

    Ok(update_listener)
}

/// Webhook implementation based on the [mod@axum] framework that can reuse
/// existing [mod@axum] server.
///
/// This function does most of the work necessary for webhook to work, it:
/// - Calls [`set_webhook`], so telegram starts sending updates our way
/// - When the update listener is [`stop`]ped, calls [`delete_webhook`]
///
/// The only missing part is running [mod@axum] server with a returned
/// [`axum::Router`].
///
/// This function is intended to be used in cases when you already have an
/// [mod@axum] server running and can reuse it for webhooks.
///
/// **Note**: in order for webhooks to work, you need to use returned
/// [`axum::Router`] in an [mod@axum] server that is bound to
/// [`options.address`].
///
/// It may also be desired to use [`with_graceful_shutdown`] with the returned
/// future in order to shutdown the server with the [`stop`] of the listener.
///
/// [`set_webhook`]: crate::payloads::SetWebhook
/// [`delete_webhook`]: crate::payloads::DeleteWebhook
/// [`stop`]: crate::stop::StopToken::stop
/// [`options.address`]: Options::address
/// [`with_graceful_shutdown`]: axum::serve::Serve::with_graceful_shutdown
///
/// ## Returns
///
/// A update listener, stop-future, axum router triplet on success.
///
/// The "stop-future" is resolved after [`stop`] is called on the stop token of
/// the returned update listener.
///
/// ## Fails
///
/// If `set_webhook()` fails.
///
/// ## See also
///
/// [`fn@axum`] for higher-level and [`axum_no_setup`] for lower-level
/// versions of this function.
pub async fn axum_to_router<R>(
    bot: R,
    mut options: Options,
) -> Result<
    (impl UpdateListener<Err = Infallible>, impl Future<Output = ()> + Send, axum::Router),
    R::Err,
>
where
    R: Requester + Send,
    <R as Requester>::DeleteWebhook: Send,
{
    use crate::{requests::Request, update_listeners::webhooks::setup_webhook};
    use futures::FutureExt;

    setup_webhook(&bot, &mut options).await?;

    let (listener, stop_flag, router) = axum_no_setup(options);

    let stop_flag = stop_flag.then(move |()| async move {
        // This assignment is needed to not require `R: Sync` since without it `&bot`
        // temporary lives across `.await` points.
        let req = bot.delete_webhook().send();
        let res = req.await;
        if let Err(err) = res {
            log::error!("Couldn't delete webhook: {}", err);
        }
    });

    Ok((listener, stop_flag, router))
}

/// Webhook implementation based on the [mod@axum] framework that doesn't
/// perform any setup work.
///
/// ## Note about the stop-future
///
/// This function returns a future that is resolved when `.stop()` is called on
/// a stop token of the update listener. Note that even if the future is not
/// used, after `.stop()` is called, update listener will not produce new
/// updates.
///
/// ## See also
///
/// [`fn@axum`] and [`axum_to_router`] for higher-level versions of this
/// function.
pub fn axum_no_setup(
    options: Options,
) -> (impl UpdateListener<Err = Infallible>, impl Future<Output = ()>, axum::Router) {
    use crate::{
        stop::{mk_stop_token, StopToken},
        update_listeners::{webhooks::tuple_first_mut, StatefulListener},
    };
    use axum::{response::IntoResponse, routing::post};
    use tokio_stream::wrappers::UnboundedReceiverStream;
    use tower_http::trace::TraceLayer;

    let (tx, rx): (UpdateSender, _) = mpsc::unbounded_channel();

    async fn telegram_request(
        State(WebhookState { secret, flag, mut tx }): State<WebhookState>,
        secret_header: XTelegramBotApiSecretToken,
        input: String,
    ) -> impl IntoResponse {
        // FIXME: use constant time comparison here
        if secret_header.0.as_deref() != secret.as_deref().map(str::as_bytes) {
            return StatusCode::UNAUTHORIZED;
        }

        let tx = match tx.get() {
            None => return StatusCode::SERVICE_UNAVAILABLE,
            // Do not process updates after `.stop()` is called even if the server is still
            // running (useful for when you need to stop the bot but can't stop the server).
            _ if flag.is_stopped() => {
                tx.close();
                return StatusCode::SERVICE_UNAVAILABLE;
            }
            Some(tx) => tx,
        };

        match serde_json::from_str::<Update>(&input) {
            Ok(mut update) => {
                // See HACK comment in
                // `teloxide_core::net::request::process_response::{closure#0}`
                if let UpdateKind::Error(value) = &mut update.kind {
                    *value = serde_json::from_str(&input).unwrap_or_default();
                }

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

    let (stop_token, stop_flag) = mk_stop_token();

    let app = axum::Router::new()
        .route(&options.path, post(telegram_request))
        .layer(TraceLayer::new_for_http())
        .with_state(WebhookState {
            tx: ClosableSender::new(tx),
            flag: stop_flag.clone(),
            secret: options.secret_token,
        });

    let stream = UnboundedReceiverStream::new(rx);

    // FIXME: this should support `hint_allowed_updates()`
    let listener = StatefulListener::new(
        (stream, stop_token),
        tuple_first_mut,
        |state: &mut (_, StopToken)| state.1.clone(),
    );

    (listener, stop_flag, app)
}

type UpdateSender = mpsc::UnboundedSender<Result<Update, std::convert::Infallible>>;
type UpdateCSender = ClosableSender<Result<Update, std::convert::Infallible>>;

#[derive(Clone)]
struct WebhookState {
    tx: UpdateCSender,
    flag: StopFlag,
    secret: Option<String>,
}

/// A terrible workaround to drop axum extension
struct ClosableSender<T> {
    origin: std::sync::Arc<std::sync::RwLock<Option<mpsc::UnboundedSender<T>>>>,
}

impl<T> Clone for ClosableSender<T> {
    fn clone(&self) -> Self {
        Self { origin: self.origin.clone() }
    }
}

impl<T> ClosableSender<T> {
    fn new(sender: mpsc::UnboundedSender<T>) -> Self {
        Self { origin: std::sync::Arc::new(std::sync::RwLock::new(Some(sender))) }
    }

    fn get(&self) -> Option<mpsc::UnboundedSender<T>> {
        self.origin.read().unwrap().clone()
    }

    fn close(&mut self) {
        self.origin.write().unwrap().take();
    }
}

struct XTelegramBotApiSecretToken(Option<Vec<u8>>);

impl<S> FromRequestParts<S> for XTelegramBotApiSecretToken {
    type Rejection = StatusCode;

    fn from_request_parts(
        req: &mut Parts,
        _state: &S,
    ) -> impl Future<Output = Result<Self, Self::Rejection>> + Send {
        use crate::update_listeners::webhooks::check_secret;

        let res = req
            .headers
            .remove("x-telegram-bot-api-secret-token")
            .map(|header| {
                check_secret(header.as_bytes())
                    .map(<_>::to_owned)
                    .map_err(|_| StatusCode::BAD_REQUEST)
            })
            .transpose()
            .map(Self);

        async { res }
    }
}
