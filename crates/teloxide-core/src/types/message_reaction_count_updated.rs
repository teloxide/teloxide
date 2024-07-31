use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageId, ReactionType};

/// This object represents reaction changes on a message with anonymous
/// reactions.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionCountUpdated {
    /// The chat containing the message
    pub chat: Chat,

    /// Unique message identifier inside the chat
    #[serde(flatten)]
    pub message_id: MessageId,

    /// Date of the change in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// List of reactions that are present on the message
    pub reactions: Vec<ReactionCount>,
}

/// Represents a reaction added to a message along with the number of times it
/// was added.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReactionCount {
    /// Type of the reaction
    pub r#type: ReactionType,

    /// Number of times the reaction was added
    pub total_count: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "message_id": 36,
            "date": 1721306391,
            "reactions": [
                {
                    "type": {
                        "type": "emoji",
                        "emoji": "🗿"
                    },
                    "total_count": 2
                },
                {
                    "type": {
                        "type": "emoji",
                        "emoji": "🌭"
                    },
                    "total_count": 1
                }
            ]
        }
        "#;
        serde_json::from_str::<MessageReactionCountUpdated>(data).unwrap();
    }
}
