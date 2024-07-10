use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotDescription {
    /// The bot's description
    pub description: String,
}
