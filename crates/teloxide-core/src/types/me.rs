use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::User;

/// Returned only in [`GetMe`].
///
/// [`GetMe`]: crate::payloads::GetMe
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Me {
    #[serde(flatten)]
    pub user: User,

    /// `true`, if the bot can be invited to groups.
    pub can_join_groups: bool,

    /// `true`, if [privacy mode] is disabled for the bot.
    ///
    /// [privacy mode]: https://core.telegram.org/bots#privacy-mode
    pub can_read_all_group_messages: bool,

    /// `true`, if the bot supports inline queries.
    pub supports_inline_queries: bool,
}

impl Me {
    /// Returns the username of the bot.
    #[must_use]
    pub fn username(&self) -> &str {
        self.user.username.as_deref().expect("Bots must have usernames")
    }

    /// Returns a username mention of this bot.
    #[must_use]
    pub fn mention(&self) -> String {
        format!("@{}", self.username())
    }

    /// Returns an URL that links to this bot in the form of `t.me/<...>`.
    #[must_use]
    pub fn tme_url(&self) -> reqwest::Url {
        format!("https://t.me/{}", self.username()).parse().unwrap()
    }
}

impl Deref for Me {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

#[cfg(test)]
mod tests {
    use crate::types::{Me, User, UserId};

    #[test]
    fn convenience_methods_work() {
        let me = Me {
            user: User {
                id: UserId(42),
                is_bot: true,
                first_name: "First".to_owned(),
                last_name: None,
                username: Some("SomethingSomethingBot".to_owned()),
                language_code: None,
                is_premium: false,
                added_to_attachment_menu: false,
            },
            can_join_groups: false,
            can_read_all_group_messages: false,
            supports_inline_queries: false,
        };

        assert_eq!(me.username(), "SomethingSomethingBot");
        assert_eq!(me.mention(), "@SomethingSomethingBot");
        assert_eq!(me.tme_url(), "https://t.me/SomethingSomethingBot".parse().unwrap());
    }
}
