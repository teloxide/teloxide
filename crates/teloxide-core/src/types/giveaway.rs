use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, CountryCode};

/// This object represents a message about a scheduled giveaway.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Giveaway {
    /// The list of chats which the user must join to participate in the
    /// giveaway.
    pub chats: Vec<Chat>,

    /// Point in time (Unix timestamp) when winners of the giveaway will be
    /// selected
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub winners_selection_date: DateTime<Utc>,

    /// The number of users which are supposed to be selected as winners of the
    /// giveaway
    pub winner_count: u32,

    /// `true`, if only users who join the chats after the giveaway started
    /// should be eligible to win
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub only_new_members: bool,

    /// `true`, if the list of giveaway winners will be visible to everyone
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub has_public_winners: bool,

    /// Description of additional giveaway prize
    pub prize_description: Option<String>,

    /// A list of two-letter [ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2) country codes indicating the
    /// countries from which eligible users for the giveaway must come. If
    /// empty, then all users can participate in the giveaway. Users with a
    /// phone number that was bought on Fragment can always participate in
    /// giveaways.
    pub country_codes: Option<Vec<CountryCode>>,

    /// The number of months the Telegram Premium subscription won from the
    /// giveaway will be active for
    pub premium_subscription_month_count: Option<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "chats": [
                {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                }
            ],
            "winners_selection_date": 1721162701,
            "winner_count": 1,
            "has_public_winners": true,
            "premium_subscription_month_count": 6
        }
        "#;
        serde_json::from_str::<Giveaway>(data).unwrap();
    }
}
