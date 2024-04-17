use serde::{Deserialize, Serialize};

/// This object represents a birthdate.
///
/// [The official docs](https://core.telegram.org/bots/api#birthdate).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: u8,
    /// Month of the user's birth; 1-12
    pub month: u8,
    /// Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}