use crate::core::types::{
    InlineKeyboardMarkup, InputMessageContent, ParseMode,
};

#[derive(Debug, Serialize, Hash, PartialEq, Eq, Clone)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}
