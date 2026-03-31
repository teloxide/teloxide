use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a contact with a phone number.
///
/// By default, this contact will be sent by the user. Alternatively, you can
/// use `input_message_content` to send a message with the specified content
/// instead of the contact.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultcachedvideo).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
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
    pub thumbnail_url: Option<reqwest::Url>,

    /// Thumbnail width.
    pub thumbnail_width: Option<u32>,

    /// Thumbnail height.
    pub thumbnail_height: Option<u32>,
}

impl InlineQueryResultContact {
    pub fn new<S1, S2, S3>(id: S1, phone_number: S2, first_name: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn phone_number<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn vcard<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.vcard = Some(val.into());
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

    #[must_use]
    pub fn thumbnail_url(mut self, val: reqwest::Url) -> Self {
        self.thumbnail_url = Some(val);
        self
    }

    #[must_use]
    pub fn thumbnail_width(mut self, val: u32) -> Self {
        self.thumbnail_width = Some(val);
        self
    }

    #[must_use]
    pub fn thumbnail_height(mut self, val: u32) -> Self {
        self.thumbnail_height = Some(val);
        self
    }
}
