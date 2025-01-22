use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

use crate::types::{ChatId, UserId};

/// A unique identifier for the target chat or username of the target channel
/// (in the format `@channelusername`).
#[derive(Clone, PartialEq, Eq, Hash)]
#[derive(Debug, Display, From)]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Recipient {
    /// A chat identifier.
    #[display("{_0}")]
    Id(ChatId),

    /// A channel username (in the format @channelusername).
    #[display("{_0}")]
    ChannelUsername(String),
}

impl From<UserId> for Recipient {
    fn from(id: UserId) -> Self {
        Self::Id(id.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_id_id_serialization() {
        let expected_json = String::from("123456");
        let actual_json = serde_json::to_string(&Recipient::Id(ChatId(123_456))).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn chat_id_channel_username_serialization() {
        let expected_json = String::from(r#""@username""#);
        let actual_json =
            serde_json::to_string(&Recipient::ChannelUsername(String::from("@username"))).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
