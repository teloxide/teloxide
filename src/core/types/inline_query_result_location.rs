use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;
use crate::core::types::parse_mode::ParseMode;

#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct InlineQueryResultLocation {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub title: String,
    pub live_period: i32,
    pub reply_markup: InlineKeyboardMarkup,
    pub input_message_content: InputMessageContent,
    pub thumb_url: String,
    pub thumb_width: i32,
    pub thumb_height: i32,

}