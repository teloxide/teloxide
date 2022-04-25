use serde::{Deserialize, Serialize};

/// This object represents a service message about a video chat ended in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VideoChatEnded {}
