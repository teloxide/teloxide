use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::with_prefix;

use crate::types::{Chat, MessageId, User};

with_prefix!(prefix_giveaway "giveaway_");

/// This object represents a message about the completion of a giveaway with
/// public winners.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveawayWinners {
    /// The chat that created the giveaway
    pub chat: Chat,

    /// Identifier of the messsage with the giveaway in the chat
    #[serde(flatten, with = "prefix_giveaway")]
    pub giveaway_message_id: MessageId,

    /// Point in time (Unix timestamp) when winners of the giveaway were
    /// selected
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub winners_selection_date: DateTime<Utc>,

    /// Total number of winners in the giveaway
    pub winner_count: u32,

    /// List of up to 100 winners of the giveaway
    pub winners: Vec<User>,

    /// The number of other chats the user had to join in order to be eligible
    /// for the giveaway
    pub additional_chat_count: Option<u16>,

    /// The number of months the Telegram Premium subscription won from the
    /// giveaway will be active for
    pub premium_subscription_month_count: Option<u8>,

    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<u32>,

    /// `true`, if only users who had joined the chats after the giveaway
    /// started were eligible to win
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub only_new_members: bool,

    /// `true`, if the giveaway was canceled because the payment for it was
    /// refunded
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub was_refunded: bool,

    /// Description of additional giveaway prize
    pub prize_description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "chat": {
                "id": -1002236736395,
                "title": "Test",
                "type": "channel"
            },
            "giveaway_message_id": 27,
            "winners_selection_date": 1721162701,
            "premium_subscription_month_count": 6,
            "winner_count": 1,
            "winners": [
                {
                    "id": 1459074222,
                    "is_bot": false,
                    "first_name": "shadowchain",
                    "username": "shdwchn10"
                }
            ]
        }
        "#;
        serde_json::from_str::<GiveawayWinners>(data).unwrap();
    }
}
