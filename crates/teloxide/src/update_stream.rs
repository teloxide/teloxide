use std::time::Duration;

use futures::StreamExt;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::sync::CancellationToken;

use crate::{
    types::{AllowedUpdate, Update},
    update_listeners::{AsUpdateStream, Polling, UpdateListener},
};

#[cfg(feature = "webhooks-axum")]
use std::net::SocketAddr;

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

    /// Cancellation token for graceful shutdown.
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
    /// ```ignore
    /// let mut stream = bot.update_stream().build().await;
    ///
    /// while let Some(Ok(update)) = stream.next().await {
    ///     match update.kind {
    ///         UpdateKind::Message(msg) => { /* ... */ },
    ///         _ => {}
    ///     }
    /// }
    /// ```
    pub async fn build(
        self,
    ) -> impl futures::Stream<Item = Result<Update, teloxide_core::RequestError>> {
        #[cfg(feature = "webhooks-axum")]
        if self.webhook_url.is_some() {
            log::warn!("webhook mode for update_stream is not yet implemented, falling back to polling");
        }

        self.build_polling().await
    }

    async fn build_polling(
        self,
    ) -> impl futures::Stream<Item = Result<Update, teloxide_core::RequestError>> {
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

        if let Some(cancel) = self.token.clone() {
            let stop_token = polling.stop_token();
            tokio::spawn(async move {
                cancel.cancelled().await;
                stop_token.stop();
            });
        }

        let (tx, rx) = mpsc::unbounded_channel();

        tokio::spawn(async move {
            let mut stream = std::pin::pin!(polling.as_stream());
            while let Some(item) = stream.next().await {
                if tx.send(item).is_err() {
                    break;
                }
            }
        });

        UnboundedReceiverStream::new(rx)
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
    /// [`Dispatcher`]: crate::dispatching::Dispatcher
    /// [`Update`]: crate::types::Update
    fn update_stream(&self) -> UpdateStreamBuilder;
}

impl UpdateStreamExt for teloxide_core::Bot {
    fn update_stream(&self) -> UpdateStreamBuilder {
        UpdateStreamBuilder::new(self.clone())
    }
}
