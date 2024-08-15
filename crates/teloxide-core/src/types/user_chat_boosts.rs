use serde::{Deserialize, Serialize};

use crate::types::ChatBoost;

/// This object represents a list of boosts added to a chat by a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserChatBoosts {
    /// The list of boosts added to the chat by the user.
    pub boosts: Vec<ChatBoost>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "boosts": [
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
            ]
        }
        "#;
        serde_json::from_str::<UserChatBoosts>(data).unwrap();
    }
}
