use std::{
    collections::HashSet,
    pin::Pin,
    sync::{LazyLock, Mutex},
    task::{Context, Poll},
    time::Duration,
};

use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

use crate::{
    types::{AllowedUpdate, Update},
    update_listeners::{AsUpdateStream, Polling},
};

#[cfg(feature = "webhooks-axum")]
use std::net::SocketAddr;

static ACTIVE_TOKENS: LazyLock<Mutex<HashSet<String>>> =
    LazyLock::new(|| Mutex::new(HashSet::new()));

pub struct UpdateStreamBuilder {
    bot: teloxide_core::Bot,
    timeout: Option<Duration>,
    limit: Option<u8>,
    allowed_updates: Option<Vec<AllowedUpdate>>,
    drop_pending_updates: bool,
    token: Option<CancellationToken>,
    #[cfg(feature = "webhooks-axum")]
    webhook_url: Option<url::Url>,
    #[cfg(feature = "webhooks-axum")]
    webhook_address: Option<SocketAddr>,
    #[cfg(feature = "webhooks-axum")]
    webhook_secret: Option<String>,
    #[cfg(feature = "webhooks-axum")]
    webhook_max_connections: Option<u8>,
}

impl UpdateStreamBuilder {
    fn new(bot: teloxide_core::Bot) -> Self {
        Self {
            bot,
            timeout: None,
            limit: None,
            allowed_updates: None,
            drop_pending_updates: false,
            token: None,
            #[cfg(feature = "webhooks-axum")]
            webhook_url: None,
            #[cfg(feature = "webhooks-axum")]
            webhook_address: None,
            #[cfg(feature = "webhooks-axum")]
            webhook_secret: None,
            #[cfg(feature = "webhooks-axum")]
            webhook_max_connections: None,
        }
    }

    /// Long polling timeout. Defaults to 10 seconds.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Maximum number of updates per poll (1-100).
    pub fn limit(mut self, limit: u8) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Filter which update types to receive.
    pub fn allowed_updates(mut self, allowed: Vec<AllowedUpdate>) -> Self {
        self.allowed_updates = Some(allowed);
        self
    }

    /// Drop all pending updates on start.
    pub fn drop_pending_updates(mut self) -> Self {
        self.drop_pending_updates = true;
        self
    }

    /// Provide an external cancellation token for graceful shutdown.
    ///
    /// If not set, an internal token is created automatically. You can
    /// always call [`UpdateStream::shutdown`] to stop the stream regardless.
    pub fn token(mut self, token: CancellationToken) -> Self {
        self.token = Some(token);
        self
    }

    #[cfg(feature = "webhooks-axum")]
    pub fn webhook(mut self, url: url::Url) -> Self {
        self.webhook_url = Some(url);
        self
    }

    #[cfg(feature = "webhooks-axum")]
    pub fn address(mut self, address: impl Into<SocketAddr>) -> Self {
        self.webhook_address = Some(address.into());
        self
    }

    #[cfg(feature = "webhooks-axum")]
    pub fn secret_token(mut self, secret: impl Into<String>) -> Self {
        self.webhook_secret = Some(secret.into());
        self
    }

    #[cfg(feature = "webhooks-axum")]
    pub fn max_connections(mut self, max: u8) -> Self {
        self.webhook_max_connections = Some(max);
        self
    }

    /// Build and return the update stream.
    ///
    /// # Panics
    ///
    /// Panics if another update stream is already active for this bot token.
    /// Only one stream can poll a given bot at a time. Multiple streams would
    /// race for updates and lose messages.
    ///
    /// Panics in webhook mode if `.address()` was not called.
    ///
    /// # Errors
    ///
    /// In webhook mode, returns an error if `set_webhook` fails.
    #[allow(unused_mut)]
    pub async fn build(mut self) -> Result<UpdateStream, teloxide_core::RequestError> {
        #[cfg(feature = "webhooks-axum")]
        if let Some(url) = self.webhook_url.take() {
            return self.build_webhook(url).await;
        }

        Ok(self.build_polling().await)
    }

    async fn build_polling(self) -> UpdateStream {
        let bot_token = self.bot.token().to_owned();

        {
            let mut active = ACTIVE_TOKENS.lock().unwrap();
            assert!(
                active.insert(bot_token.clone()),
                "another update stream is already active for this bot token, only one stream can \
                 poll a given bot at a time"
            );
        }

        let mut builder = Polling::builder(self.bot);

        builder = builder.timeout(self.timeout.unwrap_or(Duration::from_secs(10)));

        if let Some(limit) = self.limit {
            builder = builder.limit(limit);
        }

        if let Some(allowed) = self.allowed_updates {
            builder = builder.allowed_updates(allowed);
        }

        if self.drop_pending_updates {
            builder = builder.drop_pending_updates();
        }

        builder = builder.delete_webhook().await;

        let mut polling = builder.build();

        let cancel = self.token.unwrap_or_default();
        let (tx, rx) = mpsc::unbounded_channel();

        let task_cancel = cancel.clone();
        tokio::spawn(async move {
            let mut stream = std::pin::pin!(polling.as_stream());
            loop {
                let item = tokio::select! {
                    item = stream.next() => item,
                    _ = task_cancel.cancelled() => break,
                };

                match item {
                    Some(item) => {
                        if tx.send(item).is_err() {
                            break;
                        }
                    }
                    None => break,
                }
            }
        });

        UpdateStream {
            inner: UnboundedReceiverStream::new(rx),
            cancel,
            _guard: StreamGuard { bot_token },
        }
    }

    #[cfg(feature = "webhooks-axum")]
    async fn build_webhook(
        self,
        url: url::Url,
    ) -> Result<UpdateStream, teloxide_core::RequestError> {
        use crate::update_listeners::webhooks;

        let bot_token = self.bot.token().to_owned();

        {
            let mut active = ACTIVE_TOKENS.lock().unwrap();
            assert!(
                active.insert(bot_token.clone()),
                "another update stream is already active for this bot token, only one stream can \
                 poll a given bot at a time"
            );
        }

        let address = self
            .webhook_address
            .expect("webhook address is required, call .address() on the builder");

        let mut options = webhooks::Options::new(address, url);

        if self.drop_pending_updates {
            options = options.drop_pending_updates();
        }

        if let Some(secret) = self.webhook_secret {
            options = options.secret_token(secret);
        }

        if let Some(max) = self.webhook_max_connections {
            options = options.max_connections(max);
        }

        let mut listener = webhooks::axum(self.bot, options).await?;

        let cancel = self.token.unwrap_or_default();
        let (tx, rx) = mpsc::unbounded_channel();

        let task_cancel = cancel.clone();
        tokio::spawn(async move {
            let mut stream = std::pin::pin!(listener.as_stream());
            loop {
                let item = tokio::select! {
                    item = stream.next() => item,
                    _ = task_cancel.cancelled() => break,
                };

                match item {
                    Some(Ok(update)) => {
                        if tx.send(Ok(update)).is_err() {
                            break;
                        }
                    }
                    Some(Err(infallible)) => match infallible {},
                    None => break,
                }
            }
        });

        Ok(UpdateStream {
            inner: UnboundedReceiverStream::new(rx),
            cancel,
            _guard: StreamGuard { bot_token },
        })
    }
}

struct StreamGuard {
    bot_token: String,
}

impl Drop for StreamGuard {
    fn drop(&mut self) {
        let mut active = ACTIVE_TOKENS.lock().unwrap();
        active.remove(&self.bot_token);
    }
}

/// A stream of updates from Telegram.
///
/// Created by [`UpdateStreamBuilder::build`]. Implements
/// [`Stream<Item = Result<Update, RequestError>>`](futures::Stream).
///
/// Dropping this stream releases the bot token so a new stream can be created.
pub struct UpdateStream {
    inner: UnboundedReceiverStream<Result<Update, teloxide_core::RequestError>>,
    cancel: CancellationToken,
    _guard: StreamGuard,
}

impl UpdateStream {
    /// Gracefully stop the update stream.
    ///
    /// The background polling/webhook task will finish its current request
    /// and stop. The stream will yield `None` on the next poll.
    pub fn shutdown(&self) {
        self.cancel.cancel();
    }
}

impl futures::Stream for UpdateStream {
    type Item = Result<Update, teloxide_core::RequestError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.inner.poll_next_unpin(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Extension trait that adds [`update_stream`](UpdateStreamExt::update_stream)
/// to [`Bot`](teloxide_core::Bot).
pub trait UpdateStreamExt {
    /// Start building an update stream.
    ///
    /// This is a simpler alternative to [`Dispatcher`] that gives you a raw
    /// stream of [`Update`]s to match on directly, with full compile-time
    /// type safety and no dependency injection.
    ///
    /// See [issue #871](https://github.com/teloxide/teloxide/issues/871) for
    /// the motivation behind this API.
    ///
    /// Only one update stream can be active per bot token at a time.
    /// Attempting to create a second one will panic.
    ///
    /// [`Dispatcher`]: crate::dispatching::Dispatcher
    /// [`Update`]: crate::types::Update
    fn update_stream(&self) -> UpdateStreamBuilder;
}

impl UpdateStreamExt for teloxide_core::Bot {
    fn update_stream(&self) -> UpdateStreamBuilder {
        UpdateStreamBuilder::new(self.clone())
    }
}
