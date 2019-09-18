use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct InlineQueryResultGif {
    #[serde(rename = "type")]
    pub query_type: String,
    pub id: String,
    pub gif_url: String,
    pub gif_width: Option<i32>,
    pub gif_height: Option<i32>,
    pub gif_duration: Option<i32>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}