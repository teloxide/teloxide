use serde::{Deserialize, Serialize};

/// This object represents a service message about a change in auto-delete timer
/// settings.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageAutoDeleteTimerChanged {
    /// New auto-delete time for messages in the chat
    pub message_auto_delete_time: u32,
}
