use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    ForceReply, InlineKeyboardButton, InlineKeyboardMarkup, KeyboardButton, KeyboardMarkup,
    KeyboardRemove,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, From)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    Keyboard(KeyboardMarkup),
    KeyboardRemove(KeyboardRemove),
    ForceReply(ForceReply),
}

impl ReplyMarkup {
    /// Constructor for [`InlineKeyboard`] variant.
    ///
    /// This is a shortcut to
    /// `ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup::new(_))`.
    ///
    /// [`InlineKeyboard`]: ReplyMarkup::InlineKeyboard
    pub fn inline_kb<I>(inline_keyboard: I) -> Self
    where
        I: IntoIterator,
        I::Item: IntoIterator<Item = InlineKeyboardButton>,
    {
        Self::InlineKeyboard(InlineKeyboardMarkup::new(inline_keyboard))
    }

    /// Constructor for [`Keyboard`] variant.
    ///
    /// This is a shortcut to
    /// `ReplyMarkup::Keyboard(KeyboardMarkup::new(_))`.
    ///
    /// [`Keyboard`]: ReplyMarkup::Keyboard
    pub fn keyboard<K>(keyboard: K) -> Self
    where
        K: IntoIterator,
        K::Item: IntoIterator<Item = KeyboardButton>,
    {
        Self::Keyboard(KeyboardMarkup::new(keyboard))
    }

    /// Constructor for [`KeyboardRemove`] variant.
    ///
    /// This is a shortcut to
    /// `ReplyMarkup::KeyboardRemove(ReplyKeyboardRemove::new()))`.
    ///
    /// [`KeyboardRemove`]: ReplyMarkup::KeyboardRemove
    #[must_use]
    pub fn kb_remove() -> Self {
        Self::KeyboardRemove(KeyboardRemove::new())
    }

    /// Constructor for [`ForceReply`] variant.
    ///
    /// This is a shortcut to `ReplyMarkup::ForceReply(ForceReply::new())`.
    ///
    /// [`ForceReply`]: ReplyMarkup::KeyboardRemove
    #[must_use]
    pub fn force_reply() -> Self {
        Self::ForceReply(ForceReply::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_keyboard_markup() {
        let data = InlineKeyboardMarkup::default();
        let expected = ReplyMarkup::InlineKeyboard(data.clone());
        let actual: ReplyMarkup = data.into();
        assert_eq!(actual, expected)
    }
}
