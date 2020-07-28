use crate::types::ChatId;

use serde::{Deserialize, Serialize};

/// A chat message or inline message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ChatOrInlineMessage {
    Chat { chat_id: ChatId, message_id: i32 },
    Inline { inline_message_id: i32 },
}
