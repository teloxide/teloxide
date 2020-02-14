use derive_more::From;
use serde::{Deserialize, Serialize};

use crate::types::{
    ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, From)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inline_keyboard_markup() {
        let data = InlineKeyboardMarkup::default();
        let expected = ReplyMarkup::InlineKeyboardMarkup(data.clone());
        let actual: ReplyMarkup = data.into();
        assert_eq!(actual, expected)
    }
}
