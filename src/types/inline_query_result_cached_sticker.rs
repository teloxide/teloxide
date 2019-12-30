use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a link to a sticker stored on the Telegram servers. By default,
/// this sticker will be sent by the user. Alternatively, you can use
/// `input_message_content` to send a message with the specified content instead
/// of the sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedsticker).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedSticker {
    /// Unique identifier for this result, 1-64 bytes.
    pub id: String,

    /// A valid file identifier of the sticker.
    pub sticker_file_id: String,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the sticker.
    pub input_message_content: Option<InputMessageContent>,
}
