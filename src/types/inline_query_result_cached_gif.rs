use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to an animated GIF file stored on the Telegram servers. By
/// default, this animated GIF file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_message_content` to send a
/// message with specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedgif).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the GIF file.
    pub gif_file_id: String,

    /// Title for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Caption of the GIF file to be sent, 0-1024 characters.
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

    /// Content of the message to be sent instead of the GIF animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,
}
