use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageId};

/// This object describes a message that was deleted or is otherwise
/// inaccessible to the bot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InaccessibleMessage {
    /// Chat the message belonged to
    pub chat: Chat,
    /// Unique message identifier inside the chat
    pub message_id: MessageId,
}
