use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a venue. By default, the venue will be sent by the user.
/// Alternatively, you can use `input_message_content` to send a message with
/// the specified content instead of the venue.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvenue).
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVenue {
    /// Unique identifier for this result, 1-64 Bytes.
    pub id: String,

    /// Latitude of the venue location in degrees.
    pub latitude: f64,

    /// Longitude of the venue location in degrees.
    pub longitude: f64,

    /// Title of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue if known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known. (For example,
    /// `arts_entertainment/default`, `arts_entertainment/aquarium` or
    /// `food/icecream`.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the venue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_message_content: Option<InputMessageContent>,

    /// Url of the thumbnail for the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_url: Option<String>,

    /// Thumbnail width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_width: Option<i32>,

    /// Thumbnail height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb_height: Option<i32>,
}
