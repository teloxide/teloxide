/// This object represents one button of the reply keyboard.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct KeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Not::not")]
    request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    request_location: bool,
}