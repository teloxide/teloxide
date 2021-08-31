use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents a service message about a voice chat scheduled in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatScheduled {
    /// Point in time when the voice chat is supposed to be started by a chat
    /// administrator.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub start_date: DateTime<Utc>,
}
