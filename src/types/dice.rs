use serde::{Deserialize, Serialize};

use crate::requests::SendDiceEmoji;

/// This object represents an animated emoji that displays a random value.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct Dice {
    /// Emoji on which the dice throw animation is based.
    emoji: SendDiceEmoji,

    /// Value of the dice.
    ///
    /// 1-6 for [`SendDiceEmoji::Dice`] and [`SendDiceEmoji::Darts`], 1-5 for
    /// [`SendDiceEmoji::Basketball`].
    ///
    /// [`SendDiceEmoji::Dice`]: crate::types::SendDiceEmoji::Dice
    /// [`SendDiceEmoji::Darts`]:crate::types::SendDiceEmoji::Darts
    /// [`SendDiceEmoji::Basketball`]:crate::types::SendDiceEmoji::Basketball
    value: i32,
}

impl Dice {
    pub fn new(emoji: SendDiceEmoji, value: i32) -> Self {
        Self { emoji, value }
    }

    pub fn emoji(mut self, val: SendDiceEmoji) -> Self {
        self.emoji = val;
        self
    }

    pub fn value<S>(mut self, val: i32) -> Self {
        self.value = val;
        self
    }
}
