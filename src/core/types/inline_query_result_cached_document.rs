use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct InlineQueryResultCachedDocument {
    #[serde(rename = "type")]
    pub query_type: String,
    pub id: String,
    pub title: String,
    pub document_file_id: String,
    pub description: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}