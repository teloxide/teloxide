use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a voice recording in an .ogg container encoded with
/// OPUS. By default, this voice recording will be sent by the user.
/// Alternatively, you can use `input_message_content` to send a message with
/// the specified content instead of the the voice message.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvoice).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVoice {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the voice recording.
    pub voice_url: String,

    /// Recording title.
    pub title: String,

    /// Caption, 0-1024 characters.
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

    /// Recording duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_duration: Option<i32>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the voice recording.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
