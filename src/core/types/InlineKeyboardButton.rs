use serde::Deserialize;

/// This object represents one button of an inline keyboard.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct InlineKeyboardButton {
    text: String,
    #[serde(flatten)]
    kind: InlineKeyboardButtonKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum InlineKeyboardButtonKind {
    #[serde(rename = "url")]
    Url(String),
    // TODO(knsd): Url?
    #[serde(rename = "callback_data")]
    CallbackData(String), // TODO(knsd) Validate size?
    //  SwitchInlineQuery(String),
    //  SwitchInlineQueryCurrentChat(String),
    //  CallbackGame(CallbackGame),
}