use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::core::types::parse_mode::ParseMode;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct InlineQueryResultGame {
    pub id: String,
    pub game_short_name: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}