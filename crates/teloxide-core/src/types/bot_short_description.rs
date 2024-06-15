use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BotShortDescription {
    /// The bot's short description
    pub short_description: String,
}
