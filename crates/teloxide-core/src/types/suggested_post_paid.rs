use serde::{Deserialize, Serialize};

use crate::types::{Message, StarAmount};

/// Describes a service message about the failed approval of a suggested post.
/// Currently, only caused by insufficient user funds at the time of approval.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostpaid).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostPaid {
    /// Message containing the suggested post. Note that the Message object in
    /// this field will not contain the reply_to_message field even if it itself
    /// is a reply.
    pub suggested_post_message: Option<Box<Message>>,

    /// Currency in which the payment was made. Currently, one of “XTR” for
    /// Telegram Stars or “TON” for toncoins
    pub currency: String,

    /// The amount of the currency that was received by the channel in
    /// nanotoncoins; for payments in toncoins only
    pub amount: Option<u32>,

    /// The amount of Telegram Stars that was received by the channel; for
    /// payments in Telegram Stars only
    pub star_amount: Option<StarAmount>,
}
