use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a link to a sticker stored on the Telegram servers.
///
/// By default, this sticker will be sent by the user. Alternatively, you can
/// use `input_message_content` to send a message with the specified content
/// instead of the sticker.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedsticker).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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

impl InlineQueryResultCachedSticker {
    pub fn new<S1, S2>(id: S1, sticker_file_id: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            sticker_file_id: sticker_file_id.into(),
            reply_markup: None,
            input_message_content: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn sticker_file_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.sticker_file_id = val.into();
        self
    }

    #[must_use]
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    #[must_use]
    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }
}
