use serde::Deserialize;

/// This object represents a custom keyboard with reply options.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}