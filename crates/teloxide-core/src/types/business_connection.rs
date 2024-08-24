use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{BusinessConnectionId, User, UserId};

/// Describes the connection of the bot with a business account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BusinessConnection {
    /// Unique identifier of the business connection
    pub id: BusinessConnectionId,

    /// Business account user that created the business connection
    pub user: User,

    /// The user id of the private chat with the user who created the business
    /// connection
    pub user_chat_id: UserId,

    /// Date the connection was established in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub date: DateTime<Utc>,

    /// `true`, if the bot can act on behalf of the business account in chats
    /// that were active in the last 24 hours
    pub can_reply: bool,

    /// `true`, if the connection is alive
    pub is_enabled: bool,
}
