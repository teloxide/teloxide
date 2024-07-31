use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageId, ReactionType, User};

/// This object represents a change of a reaction on a message performed by a
/// user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MessageReactionUpdated {
    /// The chat containing the message the user reacted to
    pub chat: Chat,

    /// Unique identifier of the message inside the chat
    #[serde(flatten)]
    pub message_id: MessageId,

    /// The user that changed the reaction, if the user isn't anonymous
    pub user: Option<User>,

    /// The chat on behalf of which the reaction was changed, if the user is
    /// anonymous
    pub actor_chat: Option<Chat>,

    /// Date of the change in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// Previous list of reaction types that were set by the user
    pub old_reaction: Vec<ReactionType>,

    /// New list of reaction types that have been set by the user
    pub new_reaction: Vec<ReactionType>,
}

impl MessageReactionUpdated {
    #[must_use]
    pub fn actor_chat(&self) -> Option<&Chat> {
        self.actor_chat.as_ref()
    }

    #[must_use]
    pub fn user(&self) -> Option<&User> {
        self.user.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "chat": {
                "id": -1002184233434,
                "title": "Test",
                "type": "supergroup"
            },
            "message_id": 35,
            "user": {
                "id": 1459074222,
                "is_bot": false,
                "first_name": "shadowchain",
                "username": "shdwchn10",
                "language_code": "en",
                "is_premium": true
            },
            "date": 1721306082,
            "old_reaction": [],
            "new_reaction": [
                {
                    "type": "emoji",
                    "emoji": "🌭"
                }
            ]
        }
        "#;
        serde_json::from_str::<MessageReactionUpdated>(data).unwrap();
    }
}
