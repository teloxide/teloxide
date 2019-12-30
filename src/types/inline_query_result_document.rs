use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a file. By default, this file will be sent by the user
/// with an optional caption. Alternatively, you can use `input_message_content`
/// to send a message with the specified content instead of the file. Currently,
/// only **.PDF** and **.ZIP** files can be sent using this method.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultdocument).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultDocument {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// Title for the result.
    pub title: String,

    /// Caption of the document to be sent, 0-1024 characters.
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

    /// A valid URL for the file.
    pub document_url: String,

    /// Mime type of the content of the file, either `application/pdf” or
    /// `application/zip”.
    pub mime_type: String,

    /// Short description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Inline keyboard attached to the message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// URL of the thumbnail (jpeg only) for the file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i32>,
}
