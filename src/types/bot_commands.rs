use serde::{Deserialize, Serialize};

/// This object represents a bot command.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct BotCommands {
    /// Text of the command, 1-32 characters.
    /// Can contain only lowercase English letters, digits and underscores.
    pub command: String,
    
    /// Description of the command, 3-256 characters.
    pub description: String,
}
