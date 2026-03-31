use serde::{Deserialize, Serialize};

/// This object represents type of a poll, which is allowed to be created and
/// sent when the corresponding button is pressed.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum KeyboardButtonPollType {
    /// If `Quiz` is passed, the user will be allowed to create only polls in
    /// the quiz mode.
    Quiz,
    /// If `Regular` is passed, only regular polls will be allowed.
    Regular,
    /// If `Any` is passed, the user will be allowed to create a poll of any
    /// type.
    #[serde(rename = "")]
    Any,
}
