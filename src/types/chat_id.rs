use derive_more::{Display, From};
use serde::{Deserialize, Serialize};

/// A unique identifier for the target chat or username of the target channel
/// (in the format `@channelusername`).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize, Display, From)]
#[serde(untagged)]
pub enum ChatId {
    /// A chat identifier.
    #[display(fmt = "{}", _0)]
    Id(i64),

    /// A channel username (in the format @channelusername).
    #[display(fmt = "{}", _0)]
    ChannelUsername(String),
}

impl ChatId {
    pub(crate) fn is_channel(&self) -> bool {
        matches!(self.unmark(), None | Some(UnmarkedChatId::Channel(_)))
    }

    pub(crate) fn unmark(&self) -> Option<UnmarkedChatId> {
        use UnmarkedChatId::*;

        const MAX_CHANNEL_ID: i64 = -(10i64.pow(12));
        const MIN_CHANNEL_ID: i64 = MAX_CHANNEL_ID - (i32::MAX as i64);
        const MAX_USER_ID: i64 = i32::MAX as _;
        const MIN_CHAT_ID: i64 = -MAX_USER_ID;

        let res = match self {
            &Self::Id(id @ MIN_CHAT_ID..=-1) => Chat(-id as _),
            &Self::Id(id @ MIN_CHANNEL_ID..=MAX_CHANNEL_ID) => Channel((MAX_CHANNEL_ID - id) as _),
            &Self::Id(id) => {
                debug_assert!(0 < id && id < MAX_USER_ID, "malformed chat id");
                User(id as _)
            }
            Self::ChannelUsername(_) => return None,
        };

        Some(res)
    }
}

pub(crate) enum UnmarkedChatId {
    User(u32),
    Chat(u32),
    Channel(u32),
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
        let actual_json =
            serde_json::to_string(&ChatId::ChannelUsername(String::from("@username"))).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}
