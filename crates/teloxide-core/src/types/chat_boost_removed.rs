use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatBoostSource};

/// This object represents a boost removed from a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostRemoved {
    /// Chat which was boosted
    pub chat: Chat,

    // FIXME: BoostId
    /// Unique identifier of the boost
    pub boost_id: String,

    /// Point in time (Unix timestamp) when the boost was removed
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub remove_date: DateTime<Utc>,

    /// Source of the removed boost
    pub source: ChatBoostSource,
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
            "boost_id": "4506e1b7e866e33fcbde78fe1746ec3a",
            "remove_date": 1745089963,
            "source": {
                "source": "premium",
                "user": {
                    "id": 1459074222,
                    "is_bot": false,
                    "first_name": "shadowchain",
                    "username": "shdwchn10",
                    "language_code": "en",
                    "is_premium": true
                }
            }
        }
        "#;
        serde_json::from_str::<ChatBoostRemoved>(data).unwrap();
    }
}
