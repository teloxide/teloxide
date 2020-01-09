use serde::{Deserialize, Serialize};

use crate::types::{
    ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove,
};

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(markup: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboardMarkup(markup)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(markup: ForceReply) -> Self {
        ReplyMarkup::ForceReply(markup)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(markup: ReplyKeyboardMarkup) -> Self {
        ReplyMarkup::ReplyKeyboardMarkup(markup)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(markup: ReplyKeyboardRemove) -> Self {
        ReplyMarkup::ReplyKeyboardRemove(markup)
    }
}
