use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageId};

/// This object is received when messages are deleted from a connected business
/// account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: String,

    /// Information about a chat in the business account. The bot may not have
    /// access to the chat or the corresponding user.
    pub chat: Chat,

    /// The list of identifiers of deleted messages in the chat of the business
    /// account.
    pub message_ids: Vec<MessageId>,
}
