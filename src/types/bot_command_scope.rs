use serde::{Deserialize, Serialize};

use crate::types::ChatId;

/// This object represents the scope to which bot commands are applied.
///
/// ## Determining list of commands
///
/// The following algorithm is used to determine the list of commands for a
/// particular user viewing the bot menu. The first list of commands which is
/// set is returned:
///
/// ### Commands in the chat with the bot
///
/// - [`Chat`] + `language_code`
/// - [`Chat`]
/// - [`AllPrivateChats`] + `language_code`
/// - [`AllPrivateChats`]
/// - [`Default`] + `language_code`
/// - [`Default`]
///
/// ### Commands in group and supergroup chats
///
/// - [`ChatMember`] + `language_code`
/// - [`ChatMember`]
/// - [`ChatAdministrators`] + `language_code` (admins only)
/// - [`ChatAdministrators`] (admins only)
/// - [`Chat`] + `language_code`
/// - [`Chat`]
/// - [`AllChatAdministrators`] + `language_code` (admins only)
/// - [`AllChatAdministrators`] (admins only)
/// - [`AllGroupChats`] + `language_code`
/// - [`AllGroupChats`]
/// - [`Default`] + `language_code`
/// - [`Default`]
///
/// [`Default`]: BotCommandScope::Default
/// [`AllPrivateChats`]: BotCommandScope::AllPrivateChats
/// [`AllGroupChats`]: BotCommandScope::AllGroupChats
/// [`AllChatAdministrators`]: BotCommandScope::AllChatAdministrators
/// [`Chat`]: BotCommandScope::Chat
/// [`ChatAdministrators`]: BotCommandScope::ChatAdministrators
/// [`ChatMember`]: BotCommandScope::ChatMember
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum BotCommandScope {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat(#[serde(rename = "chat_id")] ChatId),
    ChatAdministrators(#[serde(rename = "chat_id")] ChatId),
    ChatMember { chat_id: ChatId, user_id: i64 },
}
