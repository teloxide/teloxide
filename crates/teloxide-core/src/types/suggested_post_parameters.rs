use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

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

/// Describes the price of a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostprice).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPrice {
    /// Currency in which the post will be paid. Currently, must be one of “XTR”
    /// for Telegram Stars or “TON” for toncoins
    pub currency: String,

    /// The amount of the currency that will be paid for the post in the
    /// smallest units of the currency, i.e. Telegram Stars or nanotoncoins.
    /// Currently, price in Telegram Stars must be between 5 and 100000, and
    /// price in nanotoncoins must be between 10000000 and 10000000000000.
    pub amount: i64,
}
