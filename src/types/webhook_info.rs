use serde::{Deserialize, Serialize};

/// Contains information about the current status of a webhook.
///
/// [The official docs](https://core.telegram.org/bots/api#webhookinfo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
