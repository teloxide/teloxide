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
