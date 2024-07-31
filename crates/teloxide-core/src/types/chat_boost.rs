use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::ChatBoostSource;

/// This object contains information about a chat boost.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatBoost {
    /// Unique identifier of the boost.
    pub boost_id: String,

    /// Point in time (Unix timestamp) when the chat was boosted.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub add_date: DateTime<Utc>,

    /// Point in time (Unix timestamp) when the boost will automatically expire,
    /// unless the booster's Telegram Premium subscription is prolonged.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub expiration_date: DateTime<Utc>,

    /// Source of the added boost.
    pub source: ChatBoostSource,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "boost_id": "4506e1b7e866e33fcbde78fe1746ec3a",
            "add_date": 1721399621,
            "expiration_date": 1745088963,
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
        serde_json::from_str::<ChatBoost>(data).unwrap();
    }
}
