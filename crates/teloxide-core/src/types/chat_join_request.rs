use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatId, ChatInviteLink, User};

/// Represents a join request sent to a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct ChatJoinRequest {
    /// Chat to which the request was sent
    pub chat: Chat,
    /// User that sent the join request
    pub from: User,
    /// Identifier of a private chat with the user who sent the join request.
    /// The bot can use this identifier for 5 minutes to send messages until
    /// the join request is processed, assuming no other administrator
    /// contacted the user.
    pub user_chat_id: ChatId,
    /// Date the request was sent in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub date: DateTime<Utc>,
    /// Bio of the user.
    pub bio: Option<String>,
    /// Chat invite link that was used by the user to send the join request
    pub invite_link: Option<ChatInviteLink>,
}
