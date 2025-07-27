use serde::{Deserialize, Serialize};

use crate::types::UserId;

/// This object represents a Telegram user or bot.
///
/// [The official docs](https://core.telegram.org/bots/api#user).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: UserId,

    /// `true`, if this user is a bot.
    pub is_bot: bool,

    /// User‘s or bot’s first name.
    pub first_name: String,

    /// User‘s or bot’s last name.
    pub last_name: Option<String>,

    /// User‘s or bot’s username.
    pub username: Option<String>,

    /// [IETF language tag] of the user's language.
    ///
    /// [IETF language tag]: https://en.wikipedia.org/wiki/IETF_language_tag
    pub language_code: Option<String>,

    /// `true`, if this user is a Telegram Premium user.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub is_premium: bool,

    /// `true`, if this user added the bot to the attachment menu.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub added_to_attachment_menu: bool,
}

impl User {
    /// Returns full name of this user, ie first and last names joined with a
    /// space.
    #[must_use]
    pub fn full_name(&self) -> String {
        match &self.last_name {
            Some(last_name) => format!("{0} {1}", self.first_name, last_name),
            None => self.first_name.clone(),
        }
    }

    /// Returns a username mention of this user. Returns `None` if
    /// `self.username.is_none()`.
    #[must_use]
    pub fn mention(&self) -> Option<String> {
        Some(format!("@{}", self.username.as_ref()?))
    }

    /// Returns an URL that links to this user in the form of
    /// `tg://user/?id=<...>`.
    #[must_use]
    pub fn url(&self) -> reqwest::Url {
        self.id.url()
    }

    /// Returns an URL that links to this user in the form of `t.me/<...>`.
    /// Returns `None` if `self.username.is_none()`.
    #[must_use]
    pub fn tme_url(&self) -> Option<reqwest::Url> {
        Some(format!("https://t.me/{}", self.username.as_ref()?).parse().unwrap())
    }

    /// Returns an URL that links to this user in the form of `t.me/<...>` or
    /// `tg://user/?id=<...>`, preferring `t.me` one when possible.
    #[must_use]
    pub fn preferably_tme_url(&self) -> reqwest::Url {
        self.tme_url().unwrap_or_else(|| self.url())
    }

    /// Returns `true` if this is the special user used by telegram bot API to
    /// denote an anonymous user that sends messages on behalf of a group.
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        // Sanity check
        debug_assert!(
            !self.id.is_anonymous()
                || (self.is_bot
                    && self.first_name == "Group"
                    && self.last_name.is_none()
                    && self.username.as_deref() == Some("GroupAnonymousBot"))
        );

        self.id.is_anonymous()
    }

    /// Returns `true` if this is the special user used by telegram bot API to
    /// denote an anonymous user that sends messages on behalf of a channel.
    #[must_use]
    pub fn is_channel(&self) -> bool {
        // Sanity check
        debug_assert!(
            !self.id.is_channel()
                || (self.is_bot
                    && self.first_name == "Channel"
                    && self.last_name.is_none()
                    && self.username.as_deref() == Some("Channel_Bot"))
        );

        self.id.is_channel()
    }

    /// Returns `true` if this is the special user used by telegram itself.
    ///
    /// It is sometimes also used as a fallback, for example when a channel post
    /// is automatically forwarded to a group, bots in a group will get a
    /// message where `from` is the Telegram user.
    #[must_use]
    pub fn is_telegram(&self) -> bool {
        // Sanity check
        debug_assert!(
            !self.id.is_telegram()
                || (!self.is_bot
                    && self.first_name == "Telegram"
                    && self.last_name.is_none()
                    && self.username.is_none())
        );

        self.id.is_telegram()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":12345,
            "is_bot":false,
            "first_name":"firstName",
            "last_name":"lastName",
            "username":"Username",
            "language_code":"ru"
        }"#;
        let expected = User {
            id: UserId(12345),
            is_bot: false,
            first_name: "firstName".to_string(),
            last_name: Some("lastName".to_string()),
            username: Some("Username".to_string()),
            language_code: Some(String::from("ru")),
            is_premium: false,
            added_to_attachment_menu: false,
        };
        let actual = serde_json::from_str::<User>(json).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn convenience_methods_work() {
        let user_a = User {
            id: UserId(43),
            is_bot: false,
            first_name: "First".to_owned(),
            last_name: Some("Last".to_owned()),
            username: Some("aaaaaaaaaaaaaaaa".to_owned()),
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };

        let user_b = User {
            id: UserId(44),
            is_bot: false,
            first_name: ".".to_owned(),
            last_name: None,
            username: None,
            language_code: None,
            is_premium: false,
            added_to_attachment_menu: false,
        };

        assert_eq!(user_a.full_name(), "First Last");
        assert_eq!(user_b.full_name(), ".");

        assert_eq!(user_a.mention(), Some("@aaaaaaaaaaaaaaaa".to_owned()));
        assert_eq!(user_b.mention(), None);

        assert_eq!(user_a.tme_url(), Some("https://t.me/aaaaaaaaaaaaaaaa".parse().unwrap()));
        assert_eq!(user_b.tme_url(), None);

        assert_eq!(user_a.preferably_tme_url(), "https://t.me/aaaaaaaaaaaaaaaa".parse().unwrap());
        assert_eq!(user_b.preferably_tme_url(), "tg://user/?id=44".parse().unwrap());
    }
}
