use serde::{Deserialize, Serialize};

/// This object represents a service message about a voice chat ended in the
/// chat.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct VoiceChatEnded;
