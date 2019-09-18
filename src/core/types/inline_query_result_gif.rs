use crate::core::types::{
    InlineKeyboardMarkup, InputMessageContent, ParseMode,
};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultGif {
    pub id: String,
    pub gif_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_width: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_height: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gif_duration: Option<i32>,
    pub thumb_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
