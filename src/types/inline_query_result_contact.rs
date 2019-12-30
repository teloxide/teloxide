use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a contact with a phone number. By default, this contact will be
/// sent by the user. Alternatively, you can use `input_message_content` to send
/// a message with the specified content instead of the contact.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedvideo).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultContact {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    /// Contact's last name.
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a [vCard], 0-2048
    /// bytes.
    ///
    /// [VCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the contact.
    pub input_message_content: Option<InputMessageContent>,

    /// Url of the thumbnail for the result.
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    pub thumb_height: Option<i32>,
}
