use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, MessageId, User};

/// This object describes the origin of a message
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MessageOrigin {
    User {
        /// Date the message was sent originally in Unix time
        #[serde(default, with = "crate::types::serde_date_from_unix_timestamp")]
        date: DateTime<Utc>,
        /// User that sent the message originally
        sender_user: User,
    },
    HiddenUser {
        /// Date the message was sent originally in Unix time
        #[serde(default, with = "crate::types::serde_date_from_unix_timestamp")]
        date: DateTime<Utc>,
        /// Name of the user that sent the message originally
        sender_user_name: String,
    },
    Chat {
        /// Date the message was sent originally in Unix time
        #[serde(default, with = "crate::types::serde_date_from_unix_timestamp")]
        date: DateTime<Utc>,
        /// Chat that sent the message originally
        sender_chat: Chat,
        /// For messages originally sent by an anonymous chat administrator,
        /// original message author signature
        author_signature: Option<String>,
    },
    Channel {
        /// Date the message was sent originally in Unix time
        #[serde(default, with = "crate::types::serde_date_from_unix_timestamp")]
        date: DateTime<Utc>,
        /// Channel chat to which the message was originally sent
        chat: Chat,
        /// Unique message identifier inside the chat
        message_id: MessageId,
        /// Signature of the original post author
        author_signature: Option<String>,
    },
}

impl MessageOrigin {
    pub fn date(&self) -> DateTime<Utc> {
        *match self {
            Self::User { date, .. } => date,
            Self::HiddenUser { date, .. } => date,
            Self::Chat { date, .. } => date,
            Self::Channel { date, .. } => date,
        }
    }
}
