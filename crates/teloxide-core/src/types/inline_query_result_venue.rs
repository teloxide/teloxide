use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent};

/// Represents a venue.
///
/// By default, the venue will be sent by the user. Alternatively, you can use
/// `input_message_content` to send a message with the specified content instead
/// of the venue.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequeryresultvenue).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known. (For example,
    /// `arts_entertainment/default`, `arts_entertainment/aquarium` or
    /// `food/icecream`.)
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,

    /// Google Places type of the venue. (See [supported types].)
    ///
    /// [supported types]: https://developers.google.com/places/web-service/supported_types
    pub google_place_type: Option<String>,

    /// [Inline keyboard] attached to the message.
    ///
    /// [Inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub reply_markup: Option<InlineKeyboardMarkup>,

    /// Content of the message to be sent instead of the venue.
    pub input_message_content: Option<InputMessageContent>,

    /// Url of the thumbnail for the result.
    pub thumbnail_url: Option<reqwest::Url>,

    /// Thumbnail width.
    pub thumbnail_width: Option<u32>,

    /// Thumbnail height.
    pub thumbnail_height: Option<u32>,
}

impl InlineQueryResultVenue {
    pub fn new<S1, S2, S3>(id: S1, latitude: f64, longitude: f64, title: S2, address: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
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

    #[must_use]
    pub fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    #[must_use]
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

    pub fn address<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.address = val.into();
        self
    }

    pub fn foursquare_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_id = Some(val.into());
        self
    }

    pub fn foursquare_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.foursquare_type = Some(val.into());
        self
    }

    pub fn google_place_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.google_place_id = Some(val.into());
        self
    }

    pub fn google_place_type<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.google_place_type = Some(val.into());
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
