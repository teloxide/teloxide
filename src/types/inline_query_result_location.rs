use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a location on a map. By default, the location will be sent by the
/// user. Alternatively, you can use `input_message_content` to send a message
/// with the specified content instead of the location.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultlocation).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
