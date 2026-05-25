use serde::{Deserialize, Serialize};

use crate::types::{ChatId, MAX_USER_ID, MIN_USER_ID};

/// Identifier of a user.
#[derive(Clone, Copy)]
#[derive(Debug, derive_more::Display)]
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
#[serde(transparent)]
pub struct UserId(pub u64);

impl UserId {
    /// Returns an URL that links to the user with this id in the form of
    /// `tg://user/?id=<...>`.
    #[must_use]
    pub fn url(self) -> reqwest::Url {
        reqwest::Url::parse(&format!("tg://user/?id={self}")).unwrap()
    }

    /// Returns `true` if this is the id of the special user used by telegram
    /// bot API to denote an anonymous user that sends messages on behalf of
    /// a group.
    #[must_use]
    pub fn is_anonymous(self) -> bool {
        // https://github.com/tdlib/td/blob/4791fb6a2af0257f6cad8396e10424a79ee5f768/td/telegram/ContactsManager.cpp#L4941-L4943
        const ANON_ID: UserId = UserId(1087968824);

        self == ANON_ID
    }

    /// Returns `true` if this is the id of the special user used by telegram
    /// bot API to denote an anonymous user that sends messages on behalf of
    /// a channel.
    #[must_use]
    pub fn is_channel(self) -> bool {
        // https://github.com/tdlib/td/blob/4791fb6a2af0257f6cad8396e10424a79ee5f768/td/telegram/ContactsManager.cpp#L4945-L4947
        const ANON_CHANNEL_ID: UserId = UserId(136817688);

        self == ANON_CHANNEL_ID
    }

    /// Returns `true` if this is the id of the special user used by telegram
    /// itself.
    ///
    /// It is sometimes also used as a fallback, for example when a channel post
    /// is automatically forwarded to a group, bots in a group will get a
    /// message where `from` is the Telegram user.
    #[must_use]
    pub fn is_telegram(self) -> bool {
        const TELEGRAM_USER_ID: UserId = UserId(777000);

        self == TELEGRAM_USER_ID
    }

    /// The smallest user id that could possibly be returned by Telegram.
    pub const MIN: Self = Self(MIN_USER_ID as u64);

    /// The largest user id that could possibly be returned by Telegram.
    pub const MAX: Self = Self(MAX_USER_ID as u64);
}

impl PartialEq<ChatId> for UserId {
    fn eq(&self, other: &ChatId) -> bool {
        // Reuse `PartialEq<UserId> for ChatId` impl
        other == self
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};

    use crate::types::UserId;

    /// Test that `UserId` is serialized as the underlying integer
    #[test]
    fn deser() {
        let user_id = S { user_id: UserId(17) };
        let json = r#"{"user_id":17}"#;

        #[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
        struct S {
            user_id: UserId,
        }

        assert_eq!(serde_json::to_string(&user_id).unwrap(), json);
        assert_eq!(user_id, serde_json::from_str(json).unwrap());
    }

    #[test]
    fn url_works() {
        let id = UserId(17);

        assert_eq!(id.url(), "tg://user/?id=17".parse().unwrap());
    }
}
