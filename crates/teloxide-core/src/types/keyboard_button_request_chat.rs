use serde::{Deserialize, Serialize};

use crate::types::{ChatAdministratorRights, RequestId};

/// This object defines the criteria used to request a suitable chat.
///
/// Information about the selected chat will be shared with the bot when the
/// corresponding button is pressed. The bot will be granted requested rights in
/// the chat if appropriate. [More about requesting chats »]
///
/// [More about requesting chats »]: https://core.telegram.org/bots/features#chat-and-user-selection
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct KeyboardButtonRequestChat {
    /// identifier of the request, which will be received back in the
    /// [`ChatShared`] object. Must be unique within the message.
    ///
    /// [`ChatShared`]: crate::types::ChatShared
    pub request_id: RequestId,

    /// Pass `true` to request a channel chat, pass `false` to request a group
    /// or a supergroup chat.
    pub chat_is_channel: bool,

    /// Pass `true` to request a forum supergroup, pass `false` to request a
    /// non-forum chat. If not specified, no additional restrictions are
    /// applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_is_forum: Option<bool>,

    /// Pass `true` to request a supergroup or a channel with a username, pass
    /// `false` to request a chat without a username. If not specified, no
    /// additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_has_username: Option<bool>,

    /// Pass `true` to request a chat owned by the user. Otherwise, no
    /// additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chat_is_created: Option<bool>,

    /// Listing the required administrator rights of the user in the chat. The
    /// rights must be a superset of bot_administrator_rights. If not specified,
    /// no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_administrator_rights: Option<ChatAdministratorRights>,

    /// Listing the required administrator rights of the bot in the chat. The
    /// rights must be a subset of user_administrator_rights. If not specified,
    /// no additional restrictions are applied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_administrator_rights: Option<ChatAdministratorRights>,

    /// Pass `true` to request a chat with the bot as a member. Otherwise, no
    /// additional restrictions are applied.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub bot_is_member: bool,

    /// Pass `true` to request the chat's title.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub request_title: bool,

    /// Pass `true` to request the chat's username.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub request_username: bool,

    /// Pass `true` to request the chat's photo.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub request_photo: bool,
}

impl KeyboardButtonRequestChat {
    /// Creates a new [`KeyboardButtonRequestChat`].
    pub fn new(request_id: RequestId, chat_is_channel: bool) -> Self {
        Self {
            request_id,
            chat_is_channel,
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: false,
            request_title: false,
            request_username: false,
            request_photo: false,
        }
    }

    /// Setter for `chat_is_forum` field.
    #[must_use]
    pub fn chat_is_forum(mut self, value: bool) -> Self {
        self.chat_is_forum = Some(value);
        self
    }

    /// Setter for `chat_has_username` field.
    #[must_use]
    pub fn chat_has_username(mut self, value: bool) -> Self {
        self.chat_has_username = Some(value);
        self
    }

    /// Setter for `chat_is_created` field.
    #[must_use]
    pub fn chat_is_created(mut self, value: bool) -> Self {
        self.chat_is_created = Some(value);
        self
    }

    /// Request a chat where the user has the specified administrator rights.
    #[must_use]
    pub fn user_administrator_rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.user_administrator_rights = Some(rights);
        self
    }

    /// Request a chat where the bot has the specified administrator rights.
    #[must_use]
    pub fn bot_administrator_rights(mut self, rights: ChatAdministratorRights) -> Self {
        self.bot_administrator_rights = Some(rights);
        self
    }

    /// Setter for `bot_is_member` field.
    #[must_use]
    pub fn bot_is_member(mut self, value: bool) -> Self {
        self.bot_is_member = value;
        self
    }

    /// Setter for `request_title` field.
    #[must_use]
    pub fn request_title(mut self) -> Self {
        self.request_title = true;
        self
    }

    /// Setter for `request_username` field.
    #[must_use]
    pub fn request_username(mut self) -> Self {
        self.request_username = true;
        self
    }

    /// Setter for `request_photo` field.
    #[must_use]
    pub fn request_photo(mut self) -> Self {
        self.request_photo = true;
        self
    }
}
