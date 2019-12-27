use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a photo. By default, this photo will be sent by the
/// user with optional caption. Alternatively, you can use
/// `input_message_content` to send a message with the specified content instead
/// of the photo.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultphoto).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultPhoto {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL of the photo. Photo must be in **jpeg** format. Photo size
    /// must not exceed 5MB.
    pub photo_url: String,

    /// URL of the thumbnail for the photo.
    pub thumb_url: String,

    /// Width of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_width: Option<i32>,

    /// Height of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_height: Option<i32>,

    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Caption of the photo to be sent, 0-1024 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the photo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
