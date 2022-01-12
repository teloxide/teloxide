use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Contains information about the current status of a webhook.
///
/// [The official docs](https://core.telegram.org/bots/api#webhookinfo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WebhookInfo {
    /// Webhook URL, `None` if webhook is not set up.
    pub url: Option<reqwest::Url>,

    /// `true`, if a custom certificate was provided for webhook certificate
    /// checks.
    pub has_custom_certificate: bool,

    /// Number of updates awaiting delivery.
    pub pending_update_count: u32,

    /// Currently used webhook IP address.
    pub ip_address: Option<String>,

    /// Time of the most recent error that happened when trying to
    /// deliver an update via webhook.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub last_error_date: Option<DateTime<Utc>>,

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
