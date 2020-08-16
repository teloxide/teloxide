use serde::{Deserialize, Serialize};

/// Contains information about the current status of a webhook.
///
/// [The official docs](https://core.telegram.org/bots/api#webhookinfo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, may be empty if webhook is not set up.
    pub url: String,

    /// `true`, if a custom certificate was provided for webhook certificate
    /// checks.
    pub has_custom_certificate: bool,

    /// Number of updates awaiting delivery.
    pub pending_update_count: u32,

    /// Unix time for the most recent error that happened when trying to
    /// deliver an update via webhook.
    pub last_error_date: Option<u64>,

    /// Error message in human-readable format for the most recent error that
    /// happened when trying to deliver an update via webhook.
    pub last_error_message: Option<String>,

    /// Maximum allowed number of simultaneous HTTPS connections to the webhook
    /// for update delivery.
    pub max_connections: Option<u32>,

    /// A list of update types the bot is subscribed to. Defaults to all update
    /// types.
    pub allowed_updates: Option<Vec<String>>,
}

impl WebhookInfo {
    pub fn new<S>(url: S, has_custom_certificate: bool, pending_update_count: u32) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            has_custom_certificate,
            pending_update_count,
            last_error_date: None,

            last_error_message: None,
            max_connections: None,
            allowed_updates: None,
        }
    }

    pub fn url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.url = val.into();
        self
    }

    pub fn has_custom_certificate(mut self, val: bool) -> Self {
        self.has_custom_certificate = val;
        self
    }

    pub fn pending_update_count(mut self, val: u32) -> Self {
        self.pending_update_count = val;
        self
    }

    pub fn last_error_date(mut self, val: u64) -> Self {
        self.last_error_date = Some(val);
        self
    }

    pub fn last_error_message<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_error_message = Some(val.into());
        self
    }

    pub fn max_connections(mut self, val: u32) -> Self {
        self.max_connections = Some(val);
        self
    }

    pub fn allowed_updates<A, S>(mut self, val: A) -> Self
    where
        A: Into<Vec<S>>,
        S: Into<String>,
    {
        self.allowed_updates = Some(val.into().into_iter().map(Into::into).collect());
        self
    }
}
