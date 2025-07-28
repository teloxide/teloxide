use serde::{Deserialize, Serialize};

/// Describes an amount of Telegram Stars.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct StarAmount {
    /// Integer amount of Telegram Stars, rounded to 0; can be negative
    pub amount: i32,

    /// The number of 1/1000000000 shares of Telegram Stars; from -999999999 to
    /// 999999999; can be negative if and only if amount is non-positive
    pub nanostar_amount: Option<i32>,
}
