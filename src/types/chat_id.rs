use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// Identifier of a chat.
///
/// Note that "a chat" here means any of group, supergroup, channel or user PM.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct ChatId(pub i64);

impl From<UserId> for ChatId {
    fn from(UserId(id): UserId) -> Self {
        Self(id as _)
    }
}

impl ChatId {
    pub(crate) fn is_channel(self) -> bool {
        matches!(self.unmark(), UnmarkedChatId::Channel(_))
    }

    pub(crate) fn unmark(self) -> UnmarkedChatId {
        use UnmarkedChatId::*;

        // https://github.com/mtcute/mtcute/blob/6933ecc3f82dd2e9100f52b0afec128af564713b/packages/core/src/utils/peer-utils.ts#L4
        const MIN_MARKED_CHANNEL_ID: i64 = -1997852516352;
        const MAX_MARKED_CHANNEL_ID: i64 = -1000000000000;
        const MIN_MARKED_CHAT_ID: i64 = MAX_MARKED_CHANNEL_ID + 1;
        const MAX_MARKED_CHAT_ID: i64 = MIN_USER_ID - 1;
        const MIN_USER_ID: i64 = 0;
        const MAX_USER_ID: i64 = (1 << 40) - 1;

        match self.0 {
            id @ MIN_MARKED_CHAT_ID..=MAX_MARKED_CHAT_ID => Group(-id as _),
            id @ MIN_MARKED_CHANNEL_ID..=MAX_MARKED_CHANNEL_ID => {
                Channel((MAX_MARKED_CHANNEL_ID - id) as _)
            }
            id @ MIN_USER_ID..=MAX_USER_ID => User(UserId(id as _)),
            id => panic!("malformed chat id: {}", id),
        }
    }
}

pub(crate) enum UnmarkedChatId {
    User(UserId),
    Group(u64),
    Channel(u64),
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::types::{ChatId, UnmarkedChatId, UserId};

    /// Test that `ChatId` is serialized as the underlying integer
    #[test]
    fn deser() {
        let chat_id = S {
            chat_id: ChatId(0xAA),
        };
        let json = r#"{"chat_id":170}"#;

        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            chat_id: ChatId,
        }

        assert_eq!(serde_json::to_string(&chat_id).unwrap(), json);
        assert_eq!(chat_id, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn user_id_unmark() {
        assert!(matches!(
            ChatId(5298363099).unmark(),
            UnmarkedChatId::User(UserId(5298363099))
        ));
    }
}
