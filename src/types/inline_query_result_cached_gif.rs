use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, ParseMode};

/// Represents a link to an animated GIF file stored on the Telegram servers. By
/// default, this animated GIF file will be sent by the user with an optional
/// caption. Alternatively, you can use `input_message_content` to send a
/// message with specified content instead of the animation.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedgif).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedGif {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier for the GIF file.
    pub gif_file_id: String,

    /// Title for the result.
    pub title: Option<String>,

    /// Caption of the GIF file to be sent, 0-1024 characters.
    pub caption: Option<String>,

    /// Send [`ParseMode::Markdown`] or [`ParseMode::HTML`], if you want
    /// Telegram apps to show [bold, italic, fixed-width text or inline
    /// URLs] in the media caption.
    ///
    /// [`ParseMode::Markdown`]: crate::types::ParseMode::Markdown
    /// [`ParseMode::HTML`]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the GIF animation.
    pub input_message_content: Option<InputMessageContent>,
}
