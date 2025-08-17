use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{BusinessBotRights, BusinessConnectionId, User, UserId};

/// Describes the connection of the bot with a business account.
#[serde_with::skip_serializing_none]
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

    /// Rights of the business bot
    pub rights: Option<BusinessBotRights>,

    /// `true`, if the connection is alive
    pub is_enabled: bool,
}
