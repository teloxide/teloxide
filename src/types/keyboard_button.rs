use serde::{Deserialize, Serialize};

/// This object represents one button of the reply keyboard. For filter text
/// buttons String can be used instead of this object to specify text of the
/// button. Optional fields are mutually exclusive.
///
/// [The official docs](https://core.telegram.org/bots/api#keyboardbutton).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct KeyboardButton {
    /// Text of the button. If none of the optional fields are used, it will
    /// be sent as a message when the button is pressed.
    pub text: String,

    /// If `true`, the user's phone number will be sent as a contact
    /// when the button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_contact: Option<bool>,

    /// If `true`, the user's current location will be sent when the
    /// button is pressed. Available in private chats only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_location: Option<bool>,
}
