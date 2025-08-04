use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Gift, MessageEntity, OwnedGiftId, UniqueGift, User};

/// This object describes a gift received and owned by a user or a chat.
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum OwnedGift {
    Regular(Box<OwnedGiftRegular>),
    Unique(Box<OwnedGiftUnique>),
}

/// Contains the list of gifts received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct OwnedGifts {
    /// The total number of gifts owned by the user or the chat
    pub total_count: u32,

    /// The list of gifts
    pub gifts: Vec<OwnedGift>,

    /// Offset for the next request. If empty, then there are no more results
    pub next_offset: Option<String>,
}

/// Describes a regular gift owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct OwnedGiftRegular {
    /// Information about the regular gift
    pub gift: Gift,

    /// Unique identifier of the gift for the bot; for gifts received on behalf
    /// of business accounts only
    pub owned_gift_id: Option<OwnedGiftId>,

    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,

    /// Date the gift was sent in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub send_date: DateTime<Utc>,

    /// Text of the message that was added to the gift
    pub text: Option<String>,

    /// Special entities that appear in the text
    pub entities: Option<Vec<MessageEntity>>,

    /// `true`, if the sender and gift text are shown only to the gift receiver;
    /// otherwise, everyone will be able to see them
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_private: bool,

    /// `true`, if the gift is displayed on the account's profile page; for
    /// gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_saved: bool,

    /// `true`, if the gift can be upgraded to a unique gift; for gifts received
    /// on behalf of business accounts only
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_be_upgraded: bool,

    /// `true`, if the gift was refunded and isn't available anymore
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub was_refunded: bool,

    /// Number of Telegram Stars that can be claimed by the receiver instead of
    /// the gift; omitted if the gift cannot be converted to Telegram Stars
    pub convert_star_count: Option<u32>,

    /// Number of Telegram Stars that were paid by the sender for the ability to
    /// upgrade the gift
    pub prepaid_upgrade_star_count: Option<u32>,
}

/// Describes a unique gift received and owned by a user or a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct OwnedGiftUnique {
    /// Information about the unique gift
    pub gift: UniqueGift,

    /// Unique identifier of the received gift for the bot; for gifts received
    /// on behalf of business accounts only
    pub owned_gift_id: Option<OwnedGiftId>,

    /// Sender of the gift if it is a known user
    pub sender_user: Option<User>,

    /// Date the gift was sent in Unix time
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "i64"))]
    pub send_date: DateTime<Utc>,

    /// `true`, if the gift is displayed on the account's profile page; for
    /// gifts received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_saved: bool,

    /// `true`, if the gift can be transferred to another owner; for gifts
    /// received on behalf of business accounts only
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub can_be_transferred: bool,

    /// Number of Telegram Stars that must be paid to transfer the gift; omitted
    /// if the bot cannot transfer the gift
    pub transfer_star_count: Option<u32>,

    /// Point in time (Unix timestamp) when the gift can be transferred. If it
    /// is in the past, then the gift can be transferred now
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    #[cfg_attr(test, schemars(with = "Option<i64>"))]
    pub next_transfer_date: Option<DateTime<Utc>>,
}
