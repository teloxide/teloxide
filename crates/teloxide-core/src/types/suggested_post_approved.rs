use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Message, SuggestedPostPrice};

/// Describes a service message about the approval of a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostapproved).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApproved {
    /// Message containing the suggested post. Note that the Message object in
    /// this field will not contain the reply_to_message field even if it itself
    /// is a reply.
    pub suggested_post_message: Option<Box<Message>>,

    /// Expected price of the post
    pub price: SuggestedPostPrice,

    /// Date when the post will be published
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub send_date: DateTime<Utc>,
}
