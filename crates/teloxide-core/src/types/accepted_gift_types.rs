use serde::{Deserialize, Serialize};

/// This object describes the types of gifts that can be gifted to a user or a
/// chat.
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct AcceptedGiftTypes {
    /// `true`, if unlimited regular gifts are accepted
    pub unlimited_gifts: bool,

    /// `true`, if limited regular gifts are accepted
    pub limited_gifts: bool,

    /// `true`, if unique gifts or gifts that can be upgraded to unique for free
    /// are accepted
    pub unique_gifts: bool,

    /// `true`, if a Telegram Premium subscription is accepted
    pub premium_subscription: bool,
}
