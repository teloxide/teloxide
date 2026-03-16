use serde::{Deserialize, Serialize};

use crate::types::{Message, SuggestedPostPrice};

/// Describes a service message about the failed approval of a suggested post.
/// Currently, only caused by insufficient user funds at the time of approval.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostapprovalfailed).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostApprovalFailed {
    /// Message containing the suggested post whose approval has failed. Note
    /// that the Message object in this field will not contain the
    /// reply_to_message field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,

    /// Expected price of the post
    pub price: SuggestedPostPrice,
}
