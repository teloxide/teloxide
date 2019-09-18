use crate::core::types::{ParseMode, InlineKeyboardMarkup, InputMessageContent};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultAudio {
    pub id: String,
    pub audio_url: String,
    pub title: String,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub performer: Option<String>,
    pub audio_duration: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}