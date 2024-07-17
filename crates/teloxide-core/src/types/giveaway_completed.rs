use serde::{Deserialize, Serialize};

use crate::types::Message;

/// This object represents a service message about the completion of a giveaway
/// without public winners.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GiveawayCompleted {
    /// Number of winners in the giveaway
    pub winner_count: u32,

    /// Number of undistributed prizes
    pub unclaimed_prize_count: Option<u32>,

    /// Message with the giveaway that was completed, if it wasn't deleted
    pub giveaway_message: Option<Box<Message>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "winner_count": 0,
            "unclaimed_prize_count": 1,
            "giveaway_message": {
                "message_id": 24,
                "sender_chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "chat": {
                    "id": -1002236736395,
                    "title": "Test",
                    "type": "channel"
                },
                "date": 1721161230,
                "giveaway": {
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
            }
        }
        "#;
        serde_json::from_str::<GiveawayCompleted>(data).unwrap();
    }
}
