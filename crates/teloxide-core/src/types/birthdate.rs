use serde::{Deserialize, Serialize};

/// Describes the birthdate of a user.
///
/// [The official docs](https://core.telegram.org/bots/api#birthdate)
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: u8,

    /// Month of the user's birth; 1-12
    pub month: u8,

    /// Optional. Year of the user's birth
    pub year: Option<u32>,
}
