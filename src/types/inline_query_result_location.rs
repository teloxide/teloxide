use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a location on a map.
///
/// By default, the location will be sent by the user. Alternatively, you can
/// use `input_message_content` to send a message with the specified content
/// instead of the location.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultlocation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQueryResultLocation {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Location latitude in degrees.
    pub latitude: f64,

    /// Location longitude in degrees.
    pub longitude: f64,

    /// Location title.
    pub title: String,

    /// Period in seconds for which the location can be updated, should be
    /// between 60 and 86400.
    pub live_period: Option<i32>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the location.
    pub input_message_content: Option<InputMessageContent>,

    /// Url of the thumbnail for the result.
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    pub thumb_height: Option<i32>,
}

impl InlineQueryResultLocation {
    pub fn new<S1, S2>(id: S1, title: S2, latitude: f64, longitude: f64) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            latitude,
            longitude,
            live_period: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    pub fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    pub fn title<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.title = val.into();
        self
    }

    pub fn live_period(mut self, val: i32) -> Self {
        self.live_period = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }

    pub fn input_message_content(mut self, val: InputMessageContent) -> Self {
        self.input_message_content = Some(val);
        self
    }

    pub fn thumb_url<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.thumb_url = Some(val.into());
        self
    }

    pub fn thumb_width(mut self, val: i32) -> Self {
        self.thumb_width = Some(val);
        self
    }

    pub fn thumb_height(mut self, val: i32) -> Self {
        self.thumb_height = Some(val);
        self
    }
}
