use serde::{Deserialize, Serialize};

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
