use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::User;

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatInviteLink {
    /// The invite link. If the link was created by another chat administrator,
    /// then the second part of the link will be replaced with “…”.
    pub invite_link: String,
    /// Creator of the link
    pub creator: User,
    /// `true`, if users joining the chat via the link need to be approved by
    /// chat administrators
    pub creates_join_request: bool,
    /// `true`, if the link is primary
    pub is_primary: bool,
    /// `true`, if the link is revoked
    pub is_revoked: bool,
    /// Invite link name
    pub name: Option<String>,
    /// Point in time when the link will expire or has been
    /// expired
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub expire_date: Option<DateTime<Utc>>,
    /// Maximum number of users that can be members of the chat simultaneously
    /// after joining the chat via this invite link; 1-99999
    pub member_limit: Option<u32>,
    /// Number of pending join requests created using this link
    pub pending_join_request_count: Option<u32>,
}
