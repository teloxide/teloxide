use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

use crate::types::{
    KeyboardButtonPollType, KeyboardButtonRequestChat, KeyboardButtonRequestUser, True, WebAppInfo,
};

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

    /// Request something from user. This is available in private chats only.
    ///
    /// See [`ButtonRequest`] documentation for options on what can be
    /// requested.
    #[serde(flatten)]
    pub request: Option<ButtonRequest>,
}

impl KeyboardButton {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { text: text.into(), request: None }
    }

    pub fn request<T>(mut self, val: T) -> Self
    where
        T: Into<ButtonRequest>,
    {
        self.request = Some(val.into());
        self
    }
}

/// Request something from user, when a button is pressed.
///
/// See individual variants documentation for more info.
#[derive(Clone, Debug, Eq, Hash, PartialEq /*, Serialize, Deserialize */)]
pub enum ButtonRequest {
    /// If this variant is used, the user's current location will be sent when
    /// the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 9 April, 2016. Older clients will display unsupported message.
    Location,

    /// If this variant is used, the user's phone number will be sent as a
    /// contact when the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 9 April, 2016. Older clients will display unsupported message.
    Contact,

    /// If this variant is used, pressing the button will open a list of
    /// suitable chats. Tapping on a chat will send its identifier to the bot in
    /// a [`chat_shared`] service message.
    ///
    /// [`chat_shared`]: crate::types::MessageKind::ChatShared
    RequestChat(KeyboardButtonRequestChat),

    /// If this variant is used, pressing the button will open a list of
    /// suitable users. Tapping on any user will send their identifier to the
    /// bot in a “user_shared” service message.
    RequestUser(KeyboardButtonRequestUser),

    /// If this variant is used, the user will be asked to create a poll and
    /// send it to the bot when the button is pressed.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 23 January, 2020. Older clients will display unsupported message.
    Poll(KeyboardButtonPollType),

    /// If this variant is used, the described Web App will be launched when the
    /// button is pressed. The Web App will be able to send a “web_app_data”
    /// service message.
    ///
    /// **Note:** this option will only work in Telegram versions released after
    /// 16 April, 2022. Older clients will display unsupported message.
    WebApp(WebAppInfo),
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

    /// If specified, pressing the button will open a list of suitable chats.
    /// Tapping on a chat will send its identifier to the bot in a “chat_shared”
    /// service message. Available in private chats only.
    #[serde(rename = "request_chat")]
    chat: Option<KeyboardButtonRequestChat>,

    /// If specified, pressing the button will open a list of suitable users.
    /// Tapping on any user will send their identifier to the bot in a
    /// “user_shared” service message. Available in private chats only.
    #[serde(rename = "request_user")]
    user: Option<KeyboardButtonRequestUser>,

    /// If specified, the user will be asked to create a poll and
    /// send it to the bot when the button is pressed. Available in private
    /// chats only.
    #[serde(rename = "request_poll")]
    poll: Option<KeyboardButtonPollType>,

    /// If specified, the described Web App will be launched when the button is
    /// pressed. The Web App will be able to send a “web_app_data” service
    /// message. Available in private chats only.
    web_app: Option<WebAppInfo>,
}

impl<'de> Deserialize<'de> for ButtonRequest {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawRequest::deserialize(deserializer)?;
        match raw {
            RawRequest { contact, location, chat, user, poll, web_app }
                if 1 < (contact.is_some() as u8
                    + location.is_some() as u8
                    + chat.is_some() as u8
                    + user.is_some() as u8
                    + poll.is_some() as u8
                    + web_app.is_some() as u8) =>
            {
                Err(D::Error::custom(
                    "`request_contact`, `request_location`, `request_chat`, `request_user`, \
                     `request_poll` and `web_app` fields are mutually exclusive",
                ))
            }
            RawRequest { contact: Some(True), .. } => Ok(Self::Contact),
            RawRequest { location: Some(True), .. } => Ok(Self::Location),
            RawRequest { chat: Some(request_chat), .. } => Ok(Self::RequestChat(request_chat)),
            RawRequest { user: Some(request_user), .. } => Ok(Self::RequestUser(request_user)),
            RawRequest { poll: Some(poll_type), .. } => Ok(Self::Poll(poll_type)),
            RawRequest { web_app: Some(web_app), .. } => Ok(Self::WebApp(web_app)),

            RawRequest {
                contact: None,
                location: None,
                chat: None,
                user: None,
                poll: None,
                web_app: None,
            } => Err(D::Error::custom(
                "Either one of `request_contact`, `request_chat`, `request_user`, \
                 `request_location`, `request_poll` and `web_app` fields is required",
            )),
        }
    }
}

impl Serialize for ButtonRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut raw = RawRequest {
            contact: None,
            location: None,
            chat: None,
            user: None,
            poll: None,
            web_app: None,
        };

        match self {
            Self::Contact => raw.contact = Some(True),
            Self::Location => raw.location = Some(True),
            Self::RequestChat(request_chat) => raw.chat = Some(request_chat.clone()),
            Self::RequestUser(request_user) => raw.user = Some(request_user.clone()),
            Self::Poll(poll_type) => raw.poll = Some(poll_type.clone()),
            Self::WebApp(web_app) => raw.web_app = Some(web_app.clone()),
        };

        raw.serialize(serializer)
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
        let button =
            KeyboardButton { text: String::from(""), request: Some(ButtonRequest::Contact) };
        let expected = r#"{"text":"","request_contact":true}"#;
        let actual = serde_json::to_string(&button).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn serialize_chat_request() {
        let button = KeyboardButton {
            text: String::from(""),
            request: Some(ButtonRequest::RequestChat(KeyboardButtonRequestChat::new(0, false))),
        };
        let expected = r#"{"text":"","request_chat":{"request_id":0,"chat_is_channel":false}}"#;
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
        let expected =
            KeyboardButton { text: String::from(""), request: Some(ButtonRequest::Contact) };
        let actual = serde_json::from_str(json).unwrap();
        assert_eq!(expected, actual);
    }
}
