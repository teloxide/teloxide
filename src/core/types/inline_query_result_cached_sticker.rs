use crate::core::types::{InlineKeyboardMarkup, InputMessageContent};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
