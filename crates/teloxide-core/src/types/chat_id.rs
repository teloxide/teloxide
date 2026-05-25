use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// Identifier of a chat.
///
/// Note that "a chat" here means any of group, supergroup, channel or user PM.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct ChatId(pub i64);

/// Bare chat id as represented in MTProto API.
///
/// In MTProto API peer ids can have different types, for example `User(1)` and
/// `Group(1)` are different chats. For bot API these peer ids are encoded in
/// such a way that they can be stored in a simple integer (ie bot API chat ids
/// have the type encoded in them). This type exposes the "bare" "peer id" of a
/// chat.
///
/// `BareChatId` can be created by [`ChatId::to_bare`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum BareChatId {
    User(UserId),
    Group(u64),
    /// Note: supergroups are considered channels.
    Channel(u64),
}

impl ChatId {
    /// Returns `true` if this is an id of a user.
    #[must_use]
    pub fn is_user(self) -> bool {
        matches!(self.to_bare(), BareChatId::User(_))
    }

    /// Returns `true` if this is an id of a group.
    ///
    /// Note: supergroup is **not** considered a group.
    #[must_use]
    pub fn is_group(self) -> bool {
        matches!(self.to_bare(), BareChatId::Group(_))
    }

    /// Returns `true` if this is an id of a channel.
    #[must_use]
    pub fn is_channel_or_supergroup(self) -> bool {
        matches!(self.to_bare(), BareChatId::Channel(_))
    }

    /// Returns user id, if this is an id of a user.
    #[must_use]
    pub fn as_user(self) -> Option<UserId> {
        match self.to_bare() {
            BareChatId::User(u) => Some(u),
            BareChatId::Group(_) | BareChatId::Channel(_) => None,
        }
    }

    /// Converts this id to "bare" MTProto peer id.
    ///
    /// See [`BareChatId`] for more.
    pub(crate) fn to_bare(self) -> BareChatId {
        use BareChatId::*;

        match self.0 {
            id @ MIN_MARKED_CHAT_ID..=MAX_MARKED_CHAT_ID => Group(-id as _),
            id @ MIN_MARKED_CHANNEL_ID..=MAX_MARKED_CHANNEL_ID => {
                Channel((MAX_MARKED_CHANNEL_ID - id) as _)
            }
            id @ MIN_USER_ID..=MAX_USER_ID => User(UserId(id as _)),
            id => panic!("malformed chat id: {id}"),
        }
    }
}

impl From<UserId> for ChatId {
    fn from(UserId(id): UserId) -> Self {
        Self(id as _)
    }
}

impl PartialEq<UserId> for ChatId {
    fn eq(&self, other: &UserId) -> bool {
        self.is_user() && *self == ChatId::from(*other)
    }
}

impl BareChatId {
    /// Converts bare chat id back to normal bot API [`ChatId`].
    #[allow(unused)]
    pub(crate) fn to_bot_api(self) -> ChatId {
        use BareChatId::*;

        match self {
            User(UserId(id)) => ChatId(id as _),
            Group(id) => ChatId(-(id as i64)),
            Channel(id) => ChatId(MAX_MARKED_CHANNEL_ID - (id as i64)),
        }
    }
}

// https://github.com/mtcute/mtcute/blob/6933ecc3f82dd2e9100f52b0afec128af564713b/packages/core/src/utils/peer-utils.ts#L4
const MIN_MARKED_CHANNEL_ID: i64 = -1997852516352;
const MAX_MARKED_CHANNEL_ID: i64 = -1000000000000;
const MIN_MARKED_CHAT_ID: i64 = MAX_MARKED_CHANNEL_ID + 1;
const MAX_MARKED_CHAT_ID: i64 = MIN_USER_ID - 1;
pub(crate) const MIN_USER_ID: i64 = 0;
pub(crate) const MAX_USER_ID: i64 = (1 << 40) - 1;

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::types::{BareChatId, ChatId, UserId};

    /// Test that `ChatId` is serialized as the underlying integer
    #[test]
    fn deser() {
        let chat_id = S { chat_id: ChatId(0xAA) };
        let json = r#"{"chat_id":170}"#;

        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            chat_id: ChatId,
        }

        assert_eq!(serde_json::to_string(&chat_id).unwrap(), json);
        assert_eq!(chat_id, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn chonky_user_id_to_bare() {
        assert!(matches!(ChatId(5298363099).to_bare(), BareChatId::User(UserId(5298363099))));
    }

    #[test]
    fn to_bare_to_bot_api_identity() {
        fn assert_identity(x: u64) {
            use BareChatId::*;

            assert_eq!(User(UserId(x)), User(UserId(x)).to_bot_api().to_bare());
            assert_eq!(Group(x), Group(x).to_bot_api().to_bare());
            assert_eq!(Channel(x), Channel(x).to_bot_api().to_bare());
        }

        // Somewhat random numbers
        let ids =
            [1, 4, 17, 34, 51, 777000, 1000000, 617136926, 1666111087, 1 << 20, (1 << 35) | 123456];

        // rust 2021 when :(
        ids.iter().copied().for_each(assert_identity);
    }

    #[test]
    fn display() {
        assert_eq!(ChatId(1).to_string(), "1");
    }

    #[test]
    fn user_id_eq() {
        assert_eq!(ChatId(12), UserId(12));
        assert_eq!(ChatId(4652762), UserId(4652762));
        assert_ne!(ChatId(17), UserId(42));

        // The user id is not well formed, so even though `-1 == max` is true,
        // we don't want user id to match
        assert_eq!(-1i64, u64::MAX as i64);
        assert_ne!(ChatId(-1), UserId(u64::MAX));
    }
}
