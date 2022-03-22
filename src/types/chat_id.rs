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

        // https://github.com/mtcute/mtcute/blob/6933ecc3f82dd2e9100f52b0afec128af564713b/packages/core/src/utils/peer-utils.ts#L4
        const MIN_MARKED_CHANNEL_ID: i64 = -1997852516352;
        const MAX_MARKED_CHANNEL_ID: i64 = -1000000000000;
        const MIN_MARKED_CHAT_ID: i64 = MAX_MARKED_CHANNEL_ID + 1;
        const MAX_MARKED_CHAT_ID: i64 = MIN_USER_ID - 1;
        const MIN_USER_ID: i64 = 0;
        const MAX_USER_ID: i64 = (1 << 40) - 1;

        let res = match self {
            &Self::Id(id @ MIN_MARKED_CHAT_ID..=MAX_MARKED_CHAT_ID) => Chat(-id as _),
            &Self::Id(id @ MIN_MARKED_CHANNEL_ID..=MAX_MARKED_CHANNEL_ID) => {
                Channel((MAX_MARKED_CHANNEL_ID - id) as _)
            }
            &Self::Id(id @ MIN_USER_ID..=MAX_USER_ID) => User(id as _),
            &Self::Id(id) => panic!("malformed chat id: {}", id),
            Self::ChannelUsername(_) => return None,
        };

        Some(res)
    }
}

pub(crate) enum UnmarkedChatId {
    User(u64),
    Chat(u64),
    Channel(u64),
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

    #[test]
    fn user_id_unmark() {
        assert!(matches!(
            ChatId::Id(5298363099).unmark(),
            Some(UnmarkedChatId::User(5298363099))
        ));
    }
}
