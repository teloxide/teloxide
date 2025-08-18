use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::SuggestedPostPrice;

/// Contains parameters of a post that is being suggested by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostparameters).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostParameters {
    /// Proposed price for the post. If the field is omitted, then the post is
    /// unpaid.
    pub price: Option<SuggestedPostPrice>,

    /// Proposed send date of the post. If specified, then the date must be
    /// between 300 second and 2678400 seconds (30 days) in the future. If the
    /// field is omitted, then the post can be published at any time within 30
    /// days at the sole discretion of the user who approves it.
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub send_date: Option<DateTime<Utc>>,
}
