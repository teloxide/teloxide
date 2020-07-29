use crate::types::User;
use serde::{Deserialize, Serialize};

/// Returned only in [`Bot::get_me`].
///
/// [`Bot::get_me`]: crate::Bot::get_me
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
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
    pub fn new(
        user: User,
        can_join_groups: bool,
        can_read_all_group_messages: bool,
        supports_inline_queries: bool,
    ) -> Self {
        Self { user, can_join_groups, can_read_all_group_messages, supports_inline_queries }
    }

    pub fn user(mut self, val: User) -> Self {
        self.user = val;
        self
    }

    #[warn(clippy::wrong_self_convention)]
    pub fn can_join_groups<S>(mut self, val: bool) -> Self {
        self.can_join_groups = val;
        self
    }

    #[warn(clippy::wrong_self_convention)]
    pub fn can_read_all_group_messages<S>(mut self, val: bool) -> Self {
        self.can_read_all_group_messages = val;
        self
    }

    #[warn(clippy::wrong_self_convention)]
    pub fn supports_inline_queries<S>(mut self, val: bool) -> Self {
        self.supports_inline_queries = val;
        self
    }
}
