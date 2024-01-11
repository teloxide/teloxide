use crate::{
    requests::{Request, Requester},
    stop::{mk_stop_token, StopFlag},
    types::{AllowedUpdate, True, Update, UpdateKind},
    update_listeners::{
        webhooks::{setup_webhook, Options},
        StopToken, UpdateListener,
    },
};

use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, status::StatusCode},
    response::IntoResponse,
    routing::post,
};
use futures::stream::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tower_http::trace::TraceLayer;

use std::{
    convert::Infallible,
    error::Error,
    fmt::{self, Debug, Display},
    future::Future,
    pin::Pin,
    task,
    task::Poll,
};

/// A webhook update listener backed by [`axum`](mod@axum).
pub struct Axum<B> {
    bot: B,
    options: Options,
    token: StopToken,
    flag: Option<StopFlag>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
    /// This is a stream of updates, coming from an `axum::Router` we've
    /// created.
    ///
    /// N.B. This field is only initialized by `take_router` and is only
    /// de-initialized by `listen`. Basically, it's a way to pass the
    /// channel from the router creation, to the listener stream.
    stream: Option<UnboundedReceiverStream<Update>>,
}

#[pin_project::pin_project]
pub struct AxumStream<'a, B: Requester> {
    axum: &'a mut Axum<B>,

    #[pin]
    inner: UnboundedReceiverStream<Update>,

    #[pin]
    webhook_deletion: Option<Option<<B::DeleteWebhook as Request>::Send>>,
}

pub enum SetupError<B: Requester> {
    Bind(hyper::Error),
    SetWebhook(B::Err),
}

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
pub fn axum<R>(bot: R, mut options: Options) -> Axum<R>
where
    R: Requester + Sync + Send + Clone + 'static,
    R::SetWebhook: Send,
    R::DeleteWebhook: Send,
{
    _ = options.get_or_gen_secret_token();
    let (token, flag) = mk_stop_token();

    Axum { bot, options, token, flag: Some(flag), allowed_updates: None, stream: None }
}

impl<B> Axum<B> {
    /// Returns a router that will listen to updates.
    ///
    /// N.B. you need to get a new router each time you re-start dispatching.
    pub fn take_router(&mut self) -> Option<axum::Router> {
        match self.stream {
            None => {
                self.reinit_stop_flag_if_needed();
                let stop_flag = self.flag.as_ref().unwrap().clone();
                let (router, stream) = create_router(&self.options, stop_flag);
                self.stream = Some(stream);
                Some(router)
            }
            Some(_) => None,
        }
    }

    fn reinit_stop_flag_if_needed(&mut self) {
        if self.flag.is_none() {
            let (token, flag) = mk_stop_token();
            self.token = token;
            self.flag = Some(flag);
        }
    }
}

impl<B> UpdateListener for Axum<B>
where
    B: Requester + Sync + Send + 'static,
    B::SetWebhook: Send,
{
    type SetupErr = SetupError<B>;
    type StreamErr = Infallible;
    type Stream<'a> = AxumStream<'a, B>;

    fn listen(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Stream<'_>, Self::SetupErr>> + Send + '_>> {
        Box::pin(async {
            // Unwrap: `stop_flag` always returns `Some`.
            self.reinit_stop_flag_if_needed();
            let stop_flag = self.flag.take().unwrap();
            let stop_token = self.token.clone();

            // If the user did not take the router themselves — spawn an axum server
            if let Some(router) = self.take_router() {
                let server = axum::Server::try_bind(&self.options.address)
                    .map_err(SetupError::Bind)?
                    .serve(router.into_make_service())
                    .with_graceful_shutdown(stop_flag);

                tokio::spawn(async move {
                    server
                        .await
                        .map_err(|err| {
                            stop_token.stop();
                            err
                        })
                        .expect("Axum server error");
                });
            }

            // Unwrap: just called `take_router`
            let stream = self.stream.take().unwrap();

            setup_webhook(&self.bot, &self.options, self.allowed_updates.clone())
                .await
                .map_err(SetupError::SetWebhook)?;

            let stream = AxumStream { axum: self, inner: stream, webhook_deletion: None };

            Ok(stream)
        })
    }

    fn stop_token(&mut self) -> StopToken {
        self.reinit_stop_flag_if_needed();
        self.token.clone()
    }

    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>) {
        // TODO: we should probably warn if there already were different allowed updates
        // before
        self.allowed_updates = Some(hint.collect());
    }
}

impl<B> Stream for AxumStream<'_, B>
where
    B: Requester,
{
    type Item = Result<Update, Infallible>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        match this.inner.poll_next(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(Some(upd)) => Poll::Ready(Some(Ok(upd))),
            Poll::Ready(None) => {
                if let Some(mut deletion) = this.webhook_deletion.as_mut().as_pin_mut() {
                    if let Some(future) = deletion.as_mut().as_pin_mut() {
                        // `Some(Some(_))` — we are currently deleting webhook
                        match future.poll(cx) {
                            Poll::Pending => Poll::Pending,

                            // We completed webhook deletion (potentially failed)
                            Poll::Ready(Ok(True) | Err(_)) => {
                                this.webhook_deletion.set(Some(None));
                                Poll::Ready(None)
                            }
                        }
                    } else {
                        // `Some(None)` — we've already deleted webhook
                        Poll::Ready(None)
                    }
                } else {
                    // `None` — we haven't yet started deleting webhook

                    this.webhook_deletion.set(Some(Some(this.axum.bot.delete_webhook().send())));

                    // Immediately wake up to poll `self.in_flight`
                    // (without this this stream becomes a zombie)
                    cx.waker().wake_by_ref();
                    Poll::Pending
                }
            }
        }
    }
}

impl<R: Requester> Debug for SetupError<R>
where
    R: Requester,
    R::Err: Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind(e) => f.debug_tuple("Bind").field(&e).finish(),
            Self::SetWebhook(e) => f.debug_tuple("SetWebhook").field(&e).finish(),
        }
    }
}

impl<R> fmt::Display for SetupError<R>
where
    R: Requester,
    R::Err: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Bind(e) => write!(f, "Error while binding an address for webhooks: {e}"),
            Self::SetWebhook(e) => write!(f, "Error while setting up webhooks: {e}"),
        }
    }
}

impl<R> Error for SetupError<R>
where
    R: Requester,
    R::Err: Error + Debug + Display + 'static,
{
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Bind(e) => Some(e),
            Self::SetWebhook(e) => Some(e),
        }
    }
}

fn create_router(
    options: &Options,
    stop_flag: StopFlag,
) -> (axum::Router, UnboundedReceiverStream<Update>) {
    let (tx, rx): (mpsc::UnboundedSender<Update>, _) = mpsc::unbounded_channel();

    let app = axum::Router::new()
        .route(options.url.path(), post(telegram_request))
        .layer(TraceLayer::new_for_http())
        .with_state(WebhookState {
            tx: ClosableSender::new(tx),
            flag: stop_flag,
            secret: options.secret_token.clone(),
        });

    (app, rx.into())
}

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

            tx.send(update).expect("Cannot send an incoming update from the webhook")
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

#[derive(Clone)]
struct WebhookState {
    tx: ClosableSender<Update>,
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

    fn from_request_parts<'l0, 'l1, 'at>(
        req: &'l0 mut Parts,
        _state: &'l1 S,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Rejection>> + Send + 'at>>
    where
        'l0: 'at,
        'l1: 'at,
        Self: 'at,
    {
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

        Box::pin(async { res }) as _
    }
}
