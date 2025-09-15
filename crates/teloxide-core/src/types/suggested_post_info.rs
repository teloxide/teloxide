use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::SuggestedPostPrice;

/// Contains information about a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostinfo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostInfo {
    /// State of the suggested post
    pub state: SuggestedPostState,

    /// Proposed price of the post. If the field is omitted, then the post is
    /// unpaid.
    pub price: Option<SuggestedPostPrice>,

    /// Proposed send date of the post. If the field is omitted, then the post
    /// can be published at any time within 30 days at the sole discretion of
    /// the user or administrator who approves it.
    #[serde(with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub send_date: Option<DateTime<Utc>>,
}

/// State of the suggested post
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedPostState {
    Pending,
    Approved,
    Declined,
}
