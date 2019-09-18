use super::inline_keyboard_markup::InlineKeyboardMarkup;
use super::input_message_content::InputMessageContent;

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct InlineQueryResultMpeg4Gif {
    #[serde(rename = "type")]
    pub query_type: String,
    pub id: String,
    pub mpeg4_url: String,
    pub mpeg4_width: Option<i32>,
    pub mpeg4_height: Option<i32>,
    pub mpeg4_duration: Option<i32>,
    pub thumb_url: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<String>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}