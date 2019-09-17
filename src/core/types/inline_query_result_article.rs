use crate::core::types::{InputMessageContent, InlineKeyboardMarkup};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct InlineQueryResultArticle {
    pub r#type: String,
    pub id: String,
    pub title: String,
    pub input_message_content: InputMessageContent,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub url: Option<String>,
    pub hide_url: Option<bool>,
    pub description: Option<String>,
    pub thumb_url: Option<String>,
    pub thumb_width: Option<u64>,
    pub thumb_height: Option<u64>,
}
