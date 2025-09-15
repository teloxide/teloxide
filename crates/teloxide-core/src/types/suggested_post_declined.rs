use serde::{Deserialize, Serialize};

use crate::types::Message;

/// Describes a service message about the rejection of a suggested post.
///
/// [The official docs](https://core.telegram.org/bots/api#suggestedpostdeclined).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedPostDeclined {
    /// Message containing the suggested post whose approval has failed. Note
    /// that the Message object in this field will not contain the
    /// reply_to_message field even if it itself is a reply.
    pub suggested_post_message: Option<Box<Message>>,

    /// Comment with which the post was declined
    pub comment: Option<String>,
}
