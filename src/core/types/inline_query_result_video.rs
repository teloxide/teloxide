use crate::core::types::{InputMessageContent, InlineKeyboardMarkup};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub video_width: Option<i32>,
    pub video_height: Option<i32>,
    pub video_duration: Option<i32>,
    pub description: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}