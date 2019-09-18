use crate::core::types::{InputMessageContent, InlineKeyboardMarkup, ParseMode};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultMpeg4Gif {
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i32>,
    pub mpeg4_height: Option<i32>,
    pub mpeg4_duration: Option<i32>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}