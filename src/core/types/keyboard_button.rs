/// This object represents one button of the reply keyboard.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct KeyboardButton {
    pub text: String,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub request_location: bool,
}