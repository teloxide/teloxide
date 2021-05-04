use serde::{Deserialize, Serialize};

/// This object represents a service message about a voice chat scheduled in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatScheduled {
    /// Point in time (Unix timestamp) when the voice chat is supposed to be
    /// started by a chat administrator.
    pub start_date: u64,
}
