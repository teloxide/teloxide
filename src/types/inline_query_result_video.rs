use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to a page containing an embedded video player or a video
/// file. By default, this video file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_messaage_content` to send a
/// message with the specified content instead of the video.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvideo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVideo {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid URL for the embedded video player or video file.
    pub video_url: String,

    /// Mime type of the content of video url, `text/html` or `video/mp4`.
    pub mime_type: String,

    /// URL of the thumbnail (jpeg only) for the video.
    pub thumb_url: String,

    /// Title for the result.
    pub title: String,

    /// Caption of the video to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// Video width.
    pub video_width: Option<i32>,

    /// Video height.
    pub video_height: Option<i32>,

    /// Video duration in seconds.
    pub video_duration: Option<i32>,

    /// Short description of the result.
    pub description: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the video. This field is
    /// **required** if [`InlineQueryResultVideo`] is used to send an HTML-page
    /// as a result (e.g., a YouTube video).
    ///
    /// [`InlineQueryResultVideo`]:
    /// crate::types::InlineQueryResultVideo
    pub input_message_content: Option<InputMessageContent>,
}
