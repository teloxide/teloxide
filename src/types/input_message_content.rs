use serde::{Deserialize, Serialize};

use crate::types::ParseMode;

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(untagged)]
/// This object represents the content of a message to be sent as a result of an
/// inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#inputmessagecontent).
pub enum InputMessageContent {
    /// Represents the content of a text message to be sent as the result of an
    /// inline query.
    Text {
        /// Text of the message to be sent, 1-4096 characters.
        message_text: String,

        /// Send [Markdown] or [HTML], if you want Telegram apps to show [bold,
        /// italic, fixed-width text or inline URLs] in the media caption.
        ///
        /// [Markdown]: https://core.telegram.org/bots/api#markdown-style
        /// [HTML]: https://core.telegram.org/bots/api#html-style
        /// [bold, italic, fixed-width text or inline URLs]: https://core.telegram.org/bots/api#formatting-options
        parse_mode: Option<ParseMode>,

        /// Disables link previews for links in the sent message.
        disable_web_page_preview: Option<bool>,
    },

    /// Represents the content of a location message to be sent as the result
    /// of an inline query.
    Location {
        /// Latitude of the location in degrees.
        latitude: f64,

        /// Longitude of the location in degrees.
        longitude: f64,

        /// Period in seconds for which the location can be updated, should be
        /// between 60 and 86400.
        live_period: Option<u32>,
    },

    /// Represents the content of a venue message to be sent as the result of
    /// an inline query.
    Venue {
        /// Latitude of the venue in degrees.
        latitude: f64,

        /// Longitude of the venue in degrees.
        longitude: f64,

        /// Name of the venue.
        title: String,

        /// Address of the venue.
        address: String,

        /// Foursquare identifier of the venue, if known.
        foursquare_id: Option<String>,

        /// Foursquare type of the venue, if known. (For example,
        /// `arts_entertainment/default`, `arts_entertainment/aquarium`
        /// or `food/icecream`.)
        foursquare_type: Option<String>,
    },

    /// Represents the content of a contact message to be sent as the result of
    /// an inline query.
    Contact {
        /// Contact's phone number.
        phone_number: String,

        /// Contact's first name.
        first_name: String,

        /// Contact's last name.
        last_name: Option<String>,

        /// Additional data about the contact in the form of a [vCard], 0-2048
        /// bytes.
        ///
        /// [vCard]: https://en.wikipedia.org/wiki/VCard
        vcard: Option<String>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_serialize() {
        let expected_json = r#"{"message_text":"text"}"#;
        let text_content = InputMessageContent::Text {
            message_text: String::from("text"),
            parse_mode: None,
            disable_web_page_preview: None,
        };

        let actual_json = serde_json::to_string(&text_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn location_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326}"#;
        let location_content = InputMessageContent::Location {
            latitude: 59.08,
            longitude: 38.4326,
            live_period: None,
        };

        let actual_json = serde_json::to_string(&location_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn venue_serialize() {
        let expected_json = r#"{"latitude":59.08,"longitude":38.4326,"title":"some title","address":"some address"}"#;
        let venue_content = InputMessageContent::Venue {
            latitude: 59.08,
            longitude: 38.4326,
            title: String::from("some title"),
            address: String::from("some address"),
            foursquare_id: None,
            foursquare_type: None,
        };

        let actual_json = serde_json::to_string(&venue_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn contact_serialize() {
        let expected_json =
            r#"{"phone_number":"+3800000000","first_name":"jhon"}"#;
        let contact_content = InputMessageContent::Contact {
            phone_number: String::from("+3800000000"),
            first_name: String::from("jhon"),
            last_name: None,
            vcard: None,
        };

        let actual_json = serde_json::to_string(&contact_content).unwrap();
        assert_eq!(expected_json, actual_json);
    }
}
