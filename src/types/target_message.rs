use crate::types::ChatId;

use serde::{Deserialize, Serialize};

/// A message in chat or inline message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TargetMessage {
    Chat { chat_id: ChatId, message_id: i32 },
    Inline { inline_message_id: String },
}

#[deprecated = "Was renamed to `TargetMessage`, please use renamed version"]
pub use TargetMessage as ChatOrInlineMessage;

impl From<String> for TargetMessage {
    fn from(inline_message_id: String) -> Self {
        Self::Inline { inline_message_id }
    }
}
