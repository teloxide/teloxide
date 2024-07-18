use serde::{Deserialize, Serialize};

use crate::types::DiceEmoji;

/// This object represents an animated emoji that displays a random value.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    pub emoji: DiceEmoji,

    /// Value of the dice.
    ///
    /// Value of the dice, 1-6 for 🎲, 🎯 and 🎳 base emoji, 1-5 for 🏀 and ⚽
    /// base emoji, 1-64 for 🎰 base emoji
    pub value: u8,
}
