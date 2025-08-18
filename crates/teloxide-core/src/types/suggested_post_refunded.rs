use serde::{Deserialize, Serialize};

use crate::types::Message;

/// Describes a service message about a payment refund for a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostrefunded).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostRefunded {
    /// Message containing the suggested post. Note that the Message object in
    /// this field will not contain the reply_to_message field even if it itself
    /// is a reply.
    pub suggested_post_message: Option<Box<Message>>,

    /// Reason for the refund.     
    pub reason: SuggestedPostRefundReason,
}

/// Currently, one of PostDeleted if the post was
/// deleted within 24 hours of being posted or removed from scheduled
/// messages without being posted, or PaymentRefunded if the payer refunded
/// their payment.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuggestedPostRefundReason {
    PostDeleted,
    PaymentRefunded,
}
