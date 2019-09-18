use crate::core::types::{InputMessageContent, InlineKeyboardMarkup, ParseMode};

#[derive(Debug, Serialize, PartialEq, Clone)]
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