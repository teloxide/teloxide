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
