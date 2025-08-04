use serde::{Deserialize, Serialize};

#[derive(Copy, Debug, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub enum DiceEmoji {
    /// "🎲" emoji. Values from 1-6. Defaults to this variant.
    #[serde(rename = "🎲")]
    Dice,

    /// "🎯" emoji. Values from 1-6.
    #[serde(rename = "🎯")]
    Darts,

    /// "🎳" emoji. Values 1-6
    #[serde(rename = "🎳")]
    Bowling,

    /// "🏀" emoji. Values from 1-5.
    #[serde(rename = "🏀")]
    Basketball,

    /// "⚽" emoji. Values 1-5
    #[serde(rename = "⚽")]
    Football,

    /// "🎰" emoji. Values 1-64
    #[serde(rename = "🎰")]
    SlotMachine,
}
