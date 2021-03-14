use serde::{Deserialize, Serialize};

use crate::types::{MessageEntity, ParseMode};

/// This object represents the content of a message to be sent as a result of an
/// inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmessagecontent).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    Text(InputMessageContentText),
    Location(InputMessageContentLocation),
    Venue(InputMessageContentVenue),
    Contact(InputMessageContentContact),
}
/// Represents the content of a text message to be sent as the result of an
/// inline query.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentText {
    /// Text of the message to be sent, 1-4096 characters.
    pub message_text: String,

    /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
    /// italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
    /// [HTML]: https://core.telegram.org/bots/api#html-style
    /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
    pub parse_mode: Option<ParseMode>,

    /// List of special entities that appear in message text, which can be
    /// specified instead of `parse_mode`.
    pub entities: Option<Vec<MessageEntity>>,

    /// Disables link previews for links in the sent message.
    pub disable_web_page_preview: Option<bool>,
}

impl InputMessageContentText {
    pub fn new<S>(message_text: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            message_text: message_text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            entities: None,
        }
    }

    pub fn message_text<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.message_text = val.into();
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn entities<C>(mut self, val: C) -> Self
    where
        C: IntoIterator<Item = MessageEntity>,
    {
        self.entities = Some(val.into_iter().collect());
        self
    }

    pub fn disable_web_page_preview(mut self, val: bool) -> Self {
        self.disable_web_page_preview = Some(val);
        self
    }
}

/// Represents the content of a location message to be sent as the result of an
/// inline query.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentLocation {
    /// Latitude of the location in degrees.
    pub latitude: f64,

    /// Longitude of the location in degrees.
    pub longitude: f64,

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<f64>,

    /// Period in seconds for which the location can be updated, should be
    /// between 60 and 86400.
    pub live_period: Option<u32>,

    /// For live locations, a direction in which the user is moving, in degrees.
    /// Must be between 1 and 360 if specified.
    pub heading: Option<u16>,

    /// For live locations, a maximum distance for proximity alerts about
    /// approaching another chat member, in meters. Must be between 1 and 100000
    /// if specified.
    pub proximity_alert_radius: Option<u32>,
}

impl InputMessageContentLocation {
    pub const fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
        }
    }

    pub const fn latitude(mut self, val: f64) -> Self {
        self.latitude = val;
        self
    }

    pub const fn longitude(mut self, val: f64) -> Self {
        self.longitude = val;
        self
    }

    pub const fn live_period(mut self, val: u32) -> Self {
        self.live_period = Some(val);
        self
    }
}

/// Represents the content of a venue message to be sent as the result of
/// an inline query.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentVenue {
    /// Latitude of the venue in degrees.
    pub latitude: f64,

    /// Longitude of the venue in degrees.
    pub longitude: f64,

    /// Name of the venue.
    pub title: String,

    /// Address of the venue.
    pub address: String,

    /// Foursquare identifier of the venue, if known.
    pub foursquare_id: Option<String>,

    /// Foursquare type of the venue, if known. (For example,
    /// `arts_entertainment/default`, `arts_entertainment/aquarium`
    /// or `food/icecream`.)
    pub foursquare_type: Option<String>,

    /// Google Places identifier of the venue.
    pub google_place_id: Option<String>,

    /// Google Places type of the venue. (See [supported types].)
    ///
    /// [supported types]: https://developers.google.com/places/web-service/supported_types
    pub google_place_type: Option<String>,
}

impl InputMessageContentVenue {
    pub fn new<S1, S2>(latitude: f64, longitude: f64, title: S1, address: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
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
}

/// Represents the content of a contact message to be sent as the result of
/// an inline query.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InputMessageContentContact {
    /// Contact's phone number.
    pub phone_number: String,

    /// Contact's first name.
    pub first_name: String,

    /// Contact's last name.
    pub last_name: Option<String>,

    /// Additional data about the contact in the form of a [vCard], 0-2048
    /// bytes.
    ///
    /// [vCard]: https://en.wikipedia.org/wiki/VCard
    pub vcard: Option<String>,
}

impl InputMessageContentContact {
    pub fn new<S1, S2>(phone_number: S1, first_name: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_serialize() {
        let expected_json = r#"{"message_text":"text"}"#;
        let text_content = InputMessageContent::Text(InputMessageContentText {
            message_text: String::from("text"),
            parse_mode: None,
            disable_web_page_preview: None,
            entities: None,
        });

        let actual_json = serde_json::to_string(&text_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn location_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326}"#;
        let location_content = InputMessageContent::Location(InputMessageContentLocation {
            latitude: 59.08,
            longitude: 38.4326,
            live_period: None,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
        });

        let actual_json = serde_json::to_string(&location_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn venue_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326,"title":"some title","address":"some address"}"#;
        let venue_content = InputMessageContent::Venue(InputMessageContentVenue {
            latitude: 59.08,
            longitude: 38.4326,
            title: String::from("some title"),
            address: String::from("some address"),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        });

        let actual_json = serde_json::to_string(&venue_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn contact_serialize() {
        let expected_json = r#"{"phone_number":"+3800000000","first_name":"jhon"}"#;
        let contact_content = InputMessageContent::Contact(InputMessageContentContact {
            phone_number: String::from("+3800000000"),
            first_name: String::from("jhon"),
            last_name: None,
            vcard: None,
        });

        let actual_json = serde_json::to_string(&contact_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
