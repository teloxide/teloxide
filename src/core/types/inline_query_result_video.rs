use crate::core::types::{
    InlineKeyboardMarkup, InputMessageContent, ParseMode,
};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultVideo {
    pub id: String,
    pub video_url: String,
    pub mime_type: String,
    pub thumb_url: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_duration: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
