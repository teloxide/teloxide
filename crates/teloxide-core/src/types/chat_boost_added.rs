use serde::{Deserialize, Serialize};

/// This object represents a service message about a user boosting a chat.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatBoostAdded {
    /// Number of boosts added by the user
    pub boost_count: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let data = r#"
        {
            "boost_count": 4
        }
        "#;
        serde_json::from_str::<ChatBoostAdded>(data).unwrap();
    }
}
