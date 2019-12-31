use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

/// A unique identifier for the target chat or username of the target channel
/// (in the format `@channelusername`).
#[derive(
    Debug, Display, PartialEq, Eq, Hash, Clone, Deserialize, Serialize, From,
)]
#[serde(untagged)]
pub enum ChatId {
    /// A chat identifier.
    #[display(fmt = "{}", _0)]
    Id(i64),

    /// A channel username (in the format @channelusername).
    #[display(fmt = "{}", _0)]
    ChannelUsername(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_id_id_serialization() {
        let expected_json = String::from(r#"123456"#);
        let actual_json = serde_json::to_string(&ChatId::Id(123_456)).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn chat_id_channel_username_serialization() {
        let expected_json = String::from(r#""@username""#);
        let actual_json = serde_json::to_string(&ChatId::ChannelUsername(
            String::from("@username"),
        ))
        .unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
