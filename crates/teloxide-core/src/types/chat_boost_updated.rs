use serde::{Deserialize, Serialize};

use crate::types::{Chat, ChatBoost};

/// This object represents a boost added to a chat or changed.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostUpdated {
    /// Chat which was boosted
    pub chat: Chat,

    /// Infomation about the chat boost
    pub boost: ChatBoost,
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
            "boost": {
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
        }
        "#;
        serde_json::from_str::<ChatBoostUpdated>(data).unwrap();
    }
}
