use serde::{Deserialize, Serialize};

use crate::types::Sticker;

/// Represents a gift that can be sent by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Gift {
    /// Unique identifier of the gift.
    pub id: String,

    /// The sticker that represents the gift.
    pub sticker: Sticker,

    /// The number of Telegram Stars that must be paid to send the sticker.
    pub star_count: u32,

    /// The number of Telegram Stars that must be paid to upgrade the gift to a unique one.
    pub upgrade_star_count: Option<u32>,

    /// The total number of the gifts of this type that can be sent; for limited gifts only.
    pub total_count: Option<u32>,

    /// The number of remaining gifts of this type that can be sent; for limited gifts only.
    pub remaining_count: Option<u32>,
}
