use crate::types::ChatId;

use serde::{Deserialize, Serialize};

/// A chat message or inline message.
#[derive(Serialize, Deserialize, Clone, Eq, Hash, PartialEq, Debug)]
#[serde(untagged)]
pub enum ChatOrInlineMessage {
    Chat { chat_id: ChatId, message_id: i32 },
    Inline { inline_message_id: i32 },
}
