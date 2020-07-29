use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum DiceEmoji {
    /// Values from 1-6. Defaults to this variant.
    #[serde(rename = "🎲")]
    Dice,

    /// Values from 1-6.
    #[serde(rename = "🎯")]
    Darts,

    /// Values from 1-5.
    #[serde(rename = "🏀")]
    Basketball,
}
