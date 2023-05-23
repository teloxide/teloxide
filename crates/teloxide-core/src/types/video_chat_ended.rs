use serde::{Deserialize, Serialize};

use crate::types::Seconds;

/// This object represents a service message about a video chat ended in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatEnded {
    /// Video chat duration in seconds.
    duration: Seconds,
}
