use crate::core::types::{InputMessageContent, InlineKeyboardMarkup};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultCachedSticker {
    pub id: String,
    pub sticker_file_id: String,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}