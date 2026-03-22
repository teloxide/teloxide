use serde::{Deserialize, Serialize};

/// Contains data sent from a Web App to the bot.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct WebAppData {
    /// The data. Be aware that a bad client can send arbitrary data in this
    /// field.
    pub data: String,

    /// Text of the web_app keyboard button, from which the Web App was opened.
    /// Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}
