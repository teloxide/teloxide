use serde::{Deserialize, Serialize};

/// This object represents an inline button that switches the current user to
/// inline mode in a chosen chat, with an optional default inline query.
///
/// [The official docs](https://core.telegram.org/bots/api#switchinlinequerychosenchat)

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct SwitchInlineQueryChosenChat {
    /// The default inline query to be inserted in the input field. If left
    /// empty, only the bot's username will be inserted
    pub query: Option<String>,
    /// True, if private chats with users can be chosen
    #[serde(default)]
    pub allow_user_chats: bool,
    /// True, if private chats with bots can be chosen
    #[serde(default)]
    pub allow_bot_chats: bool,
    /// True, if group and supergroup chats can be chosen
    #[serde(default)]
    pub allow_group_chats: bool,
    /// True, if channel chats can be chosen
    #[serde(default)]
    pub allow_channel_chats: bool,
}
