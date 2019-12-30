use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without
/// sound). By default, this animated MPEG-4 file will be sent by the user with
/// optional caption. Alternatively, you can use `input_message_content` to send
/// a message with the specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultmpeg4gif).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the MP4 file. File size must not exceed 1MB.
    pub mpeg4_url: String,

    /// Video width.
    pub mpeg4_width: Option<i32>,

    /// Video height.
    pub mpeg4_height: Option<i32>,

    /// Video duration.
    pub mpeg4_duration: Option<i32>,

    /// URL of the static thumbnail (jpeg or gif) for the result.
    pub thumb_url: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video animation.
    pub input_message_content: Option<InputMessageContent>,
}
