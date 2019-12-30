use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to an MP3 audio file. By default, this audio file will be
/// sent by the user. Alternatively, you can use `input_message_content` to send
/// a message with the specified content instead of the audio.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultaudio).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultAudio {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the audio file.
    pub audio_url: String,

    /// Title.
    pub title: String,

    /// Caption, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Performer.
    pub performer: Option<String>,

    /// Audio duration in seconds.
    pub audio_duration: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the audio.
    pub input_message_content: Option<InputMessageContent>,
}
