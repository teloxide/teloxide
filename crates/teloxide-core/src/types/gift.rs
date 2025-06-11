use std::hash::{Hash, Hasher};

use serde::{Deserialize, Serialize};

use crate::types::Sticker;

/// This object represents a list of gifts.
#[derive(Clone, Debug)]
#[derive(PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Gifts {
    /// The list of gifts.
    pub gifts: Vec<Gift>,
}


/// Represents a gift that can be sent by the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
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


// Manual implementation of Eq, PartialEq, and Hash for Gift,
// because the `Sticker` type does not implement `Hash` or `Eq`.
// We assume that `sticker` is a display-only field and doesn't affect
// logical identity or comparisons of Gift instances.
// Therefore, we exclude it from hashing and equality.

impl Eq for Gift {}

impl Hash for Gift {
    /// Hash all fields except `sticker`, which does not implement `Hash`.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.star_count.hash(state);
        self.upgrade_star_count.hash(state);
        self.total_count.hash(state);
        self.remaining_count.hash(state);
    }
}

impl PartialEq for Gift {
    /// Compare all fields except `sticker`, which does not implement `Eq`.
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.star_count == other.star_count
            && self.upgrade_star_count == other.upgrade_star_count
            && self.sticker == other.sticker
            && self.total_count == other.total_count
            && self.remaining_count == other.remaining_count
    }
}
