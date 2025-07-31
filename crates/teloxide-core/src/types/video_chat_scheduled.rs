use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat scheduled in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct VideoChatScheduled {
    /// Point in time when the video chat is supposed to be started by a chat
    /// administrator.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub start_date: DateTime<Utc>,
}
