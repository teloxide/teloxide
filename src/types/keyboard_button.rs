use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{KeyboardButtonPollType, True};

/// This object represents one button of the reply keyboard.
///
/// For filter text buttons String can be used instead of this object to specify
/// text of the button.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbutton).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will
    /// be sent as a message when the button is pressed.
    pub text: String,

    /// Request something from user.
    /// - If `Some(Contact)`, the user's phone number will be sent as a contact
    ///   when the button is pressed. Available in private chats only
    /// - If `Some(Location)`, the user's current location will be sent when
    ///   the button is pressed. Available in private chats only
    #[serde(flatten)]
    pub request: Option<ButtonRequest>,
}

impl KeyboardButton {
    /// Creates `KeyboardButton` with the provided `text` and all the other
    /// fields set to `None`.
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { text: text.into(), request: None }
    }

    pub fn request<T>(mut self, val: T) -> Self
    where
        T: Into<Option<ButtonRequest>>,
    {
        self.request = val.into();
        self
    }
}

// Serialize + Deserialize are implemented by hand
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ButtonRequest {
    Location,
    Contact,
    KeyboardButtonPollType(KeyboardButtonPollType),
}

/// Helper struct for (de)serializing [`ButtonRequest`](ButtonRequest)
#[serde_with_macros::skip_serializing_none]
#[derive(Serialize, Deserialize)]
struct RawRequest {
    /// If `true`, the user's phone number will be sent as a contact
    /// when the button is pressed. Available in private chats only.
    #[serde(rename = "request_contact")]
    contact: Option<True>,

    /// If `true`, the user's current location will be sent when the
    /// button is pressed. Available in private chats only.
    #[serde(rename = "request_location")]
    location: Option<True>,

    /// If specified, the user will be asked to create a poll and
    /// send it to the bot when the button is pressed. Available in private
    /// chats only.
    #[serde(rename = "request_poll")]
    poll: Option<KeyboardButtonPollType>,
}

impl<'de> Deserialize<'de> for ButtonRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRequest::deserialize(deserializer)?;
        match raw {
            RawRequest {
                contact: Some(_),
                location: Some(_),
                poll: Some(_),
            } => Err(D::Error::custom(
                "`request_contact` and `request_location` fields are mutually \
                 exclusive, but both were provided",
            )),
            RawRequest { contact: Some(_), .. } => Ok(Self::Contact),
            RawRequest { location: Some(_), .. } => Ok(Self::Location),
            RawRequest { poll: Some(poll_type), .. } => {
                Ok(Self::KeyboardButtonPollType(poll_type))
            }
            _ => Err(D::Error::custom(
                "Either one of `request_contact` and `request_location` \
                 fields is required",
            )),
        }
    }
}

impl Serialize for ButtonRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Contact => {
                RawRequest { contact: Some(True), location: None, poll: None }
                    .serialize(serializer)
            }
            Self::Location => {
                RawRequest { contact: None, location: Some(True), poll: None }
                    .serialize(serializer)
            }
            Self::KeyboardButtonPollType(poll_type) => RawRequest {
                contact: None,
                location: None,
                poll: Some(poll_type.clone()),
            }
            .serialize(serializer),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_no_request() {
        let button = KeyboardButton { text: String::from(""), request: None };
        let expected = r#"{"text":""}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_request_contact() {
        let button = KeyboardButton {
            text: String::from(""),
            request: Some(ButtonRequest::Contact),
        };
        let expected = r#"{"text":"","request_contact":true}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_no_request() {
        let json = r#"{"text":""}"#;
        let expected = KeyboardButton { text: String::from(""), request: None };
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn deserialize_request_contact() {
        let json = r#"{"text":"","request_contact":true}"#;
        let expected = KeyboardButton {
            text: String::from(""),
            request: Some(ButtonRequest::Contact),
        };
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
