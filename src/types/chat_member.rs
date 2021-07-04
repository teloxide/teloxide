use std::ops::Deref;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object contains information about one member of the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatmember).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatMember {
    /// Information about the user.
    pub user: User,

    /// The member's status in the chat.
    #[serde(flatten)]
    pub kind: ChatMemberKind,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "status")]
pub enum ChatMemberKind {
    #[serde(rename = "creator")]
    Owner(Owner),
    Administrator(Administrator),
    Member,
    Restricted(Restricted),
    Left,
    #[serde(rename = "kicked")]
    Banned(Banned),
}

/// Owner of the group. This struct is part of the [`ChatMemberKind`] enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    /// Custom title for this user.
    pub custom_title: Option<String>,

    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
}

/// Administrator of the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Administrator {
    /// Custom title for this user.
    pub custom_title: Option<String>,

    /// `true` if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    /// `true` if the bot is allowed to edit administrator privileges of that
    /// user.
    pub can_be_edited: bool,

    /// `true` if the administrator can access the chat event log, chat
    /// statistics, message statistics in channels, see channel members, see
    /// anonymous administrators in supergroups and ignore slow mode. Implied by
    /// any other administrator privilege
    pub can_manage_chat: bool,

    /// `true` if the administrator can change the chat title, photo and other
    /// settings.
    pub can_change_info: bool,

    /// `true` if the administrator can post in the channel, channels only.
    pub can_post_messages: Option<bool>,

    /// `true` if the administrator can edit messages of other users and can pin
    /// messages, channels only.
    pub can_edit_messages: Option<bool>,

    /// `true` if the administrator can delete messages of other users.
    pub can_delete_messages: bool,

    /// `true` if the administrator can manage voice chats.
    pub can_manage_voice_chats: bool,

    /// `true` if the administrator can invite new users to the chat.
    pub can_invite_users: bool,

    /// `true` if the administrator can restrict, ban or unban chat members.
    pub can_restrict_members: bool,

    /// `true` if the administrator can pin messages, supergroups only.
    pub can_pin_messages: Option<bool>,

    /// `true` if the administrator can add new administrators with a subset of
    /// his own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed
    /// by the user).
    pub can_promote_members: bool,
}

/// User, restricted in the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Restricted {
    /// Date when restrictions will be lifted for this user.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub until_date: DateTime<Utc>,

    /// `true` if the user can send text messages, contacts, locations and
    /// venues.
    pub can_send_messages: bool,

    /// `true` if the user is allowed to send audios, documents, photos, videos,
    /// video notes and voice notes.
    pub can_send_media_messages: bool,

    /// `true` if the user is allowed to send animations, games, stickers and
    /// use inline bots.
    pub can_send_other_messages: bool,

    /// `true` if the user is allowed to add web page previews to their
    /// messages.
    pub can_add_web_page_previews: bool,
}

/// User that was banned in the chat and can't return to it or view chat
/// messages. This struct is part of the [`ChatMemberKind`] enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Banned {
    /// Date when restrictions will be lifted for this user.
    #[serde(with = "crate::types::serde_date_from_unix_timestamp")]
    pub until_date: DateTime<Utc>,
}

/// This allows calling [`ChatMemberKind`]'s methods directly on [`ChatMember`].
///
/// ```no_run
/// use teloxide_core::types::ChatMember;
///
/// let member: ChatMember = todo!();
///
/// let _ = member.status();
/// let _ = member.kind.status();
/// ```
impl Deref for ChatMember {
    type Target = ChatMemberKind;

    fn deref(&self) -> &Self::Target {
        &self.kind
    }
}

/// Simple methods for checking a user status.
impl ChatMemberKind {
    /// Returns chat member status.
    pub fn status(&self) -> ChatMemberStatus {
        match self {
            ChatMemberKind::Owner(_) => ChatMemberStatus::Owner,
            ChatMemberKind::Administrator(_) => ChatMemberStatus::Administrator,
            ChatMemberKind::Member => ChatMemberStatus::Member,
            ChatMemberKind::Restricted(_) => ChatMemberStatus::Restricted,
            ChatMemberKind::Left => ChatMemberStatus::Left,
            ChatMemberKind::Banned(_) => ChatMemberStatus::Banned,
        }
    }

    /// Returns `true` if the user is the [owner] of the given chat.
    ///
    /// [owner]: ChatMemberKind::Owner
    pub fn is_owner(&self) -> bool {
        matches!(self, Self::Owner { .. })
    }

    /// Returns `true` if the user is an [administrator] of the given chat.
    ///
    /// [administrator]: ChatMemberKind::Administrator
    ///
    /// **Note**: this function **doesn't** return `true` if the user is the
    /// owner of the given chat. See also: [`is_privileged`].
    ///
    /// [`is_privileged`]: ChatMemberKind::is_privileged
    pub fn is_administrator(&self) -> bool {
        matches!(self, Self::Administrator { .. })
    }

    /// Returns `true` if the user is a common [member] of the given chat.
    ///
    /// [member]: ChatMemberKind::Member
    pub fn is_member(&self) -> bool {
        matches!(self, Self::Member { .. })
    }

    /// Returns `true` if the user is [restricted] in the given chat.
    ///
    /// [restricted]: ChatMemberKind::Restricted
    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted { .. })
    }

    /// Returns `true` if the user [left] the given chat.
    ///
    /// [left]: ChatMemberKind::Left
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left { .. })
    }

    /// Returns `true` if the user is [banned] in the given chat.
    ///
    /// [banned]: ChatMemberKind::Banned
    pub fn is_banned(&self) -> bool {
        matches!(self, Self::Banned { .. })
    }

    /// Returns `true` if the user is [kicked] from the given chat.
    ///
    /// [kicked]: ChatMemberKind::Banned
    #[deprecated = "use `is_banned` instead"]
    pub fn is_kicked(&self) -> bool {
        self.is_banned()
    }

    /// Returns `true` if the user is the [creator] (owner) of the given chat.
    ///
    /// [creator]: ChatMemberKind::Owner
    #[deprecated = "use `is_owner` instead"]
    pub fn is_creator(&self) -> bool {
        self.is_owner()
    }
}

/// Compound methods for checking a user status.
impl ChatMemberKind {
    /// Returns `true` if the user is privileged in the given chat. i.e. if the
    /// user is either the [owner] or an [administrator] in the given chat.
    ///
    /// [owner]: ChatMemberKind::Owner
    /// [administrator]: ChatMemberKind::Administrator
    pub fn is_privileged(&self) -> bool {
        self.is_administrator() || self.is_owner()
    }

    /// Returns `true` if the user is currently present in the chat. i.e. if the
    /// user **hasn't** [left] or been [banned].
    ///
    /// [left]: ChatMemberKind::Left
    /// [banned]: ChatMemberKind::Banned
    pub fn is_present(&self) -> bool {
        !(self.is_left() || self.is_banned())
    }
}

impl ChatMemberKind {
    /// Getter for [`Administrator::custom_title`] and [`Owner::custom_title`]
    /// fields.
    pub fn custom_title(&self) -> Option<&str> {
        match &self {
            Self::Administrator(Administrator { custom_title, .. })
            | Self::Owner(Owner { custom_title, .. }) => custom_title.as_deref(),
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => None,
        }
    }

    /// Returns `true` if the user's presence in the chat is hidden.
    ///
    /// I.e. returns `true` if the user is the owner of the chat or an
    /// administrator in the chat and has the [`can_manage_chat`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_manage_chat`]: Administrator::can_manage_chat
    pub fn is_anonymous(&self) -> bool {
        match self {
            Self::Owner(Owner { is_anonymous, .. })
            | Self::Administrator(Administrator { is_anonymous, .. }) => *is_anonymous,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Getter for [`Restricted::until_date`] and [`Banned::until_date`] fields.
    pub fn until_date(&self) -> Option<DateTime<Utc>> {
        match &self {
            Self::Owner(_) | Self::Administrator(_) | Self::Member | Self::Left => None,
            Self::Restricted(Restricted { until_date, .. })
            | Self::Banned(Banned { until_date, .. }) => Some(*until_date),
        }
    }
}

/// Methods for checking admin privileges.
impl ChatMemberKind {
    /// Returns `true` if the user is an administrator in the given chat and the
    /// bot is allowed to edit administrator privileges of that user.
    pub fn can_be_edited(&self) -> bool {
        match self {
            Self::Administrator(Administrator { can_be_edited, .. }) => *can_be_edited,
            // Owner can't ever be edited by any bot.
            Self::Owner(_) | Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => {
                false
            }
        }
    }

    /// Returns `true` if the user can access the chat event log, chat
    /// statistics, message statistics in channels, see channel members, see
    /// anonymous administrators in supergroups and ignore slow mode. Implied by
    /// any other administrator privilege.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has [`can_manage_chat`]
    ///   privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_manage_chat`]: Administrator::can_manage_chat
    pub fn can_manage_chat(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_manage_chat, ..
            }) => *can_manage_chat,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => true,
        }
    }

    /// Returns `true` if the user can change the chat title, photo and other
    /// settings.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the [`can_change_info`]
    ///   privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_change_info`]: Administrator::can_change_info
    pub fn can_change_info(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_change_info, ..
            }) => *can_change_info,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can post in the channel, channels only.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat (even if the chat is not a channel)
    /// - is an administrator in the given chat and has [`can_post_messages`]
    ///   privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_post_messages`]: Administrator::can_post_messages
    pub fn can_post_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_post_messages, ..
            }) => can_post_messages.unwrap_or_default(),
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can edit messages of other users and can pin
    /// messages, channels only.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat (even if the chat is not a channel)
    /// - is an administrator in the given chat and has the
    ///   [`can_edit_messages`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_edit_messages`]: Administrator::can_edit_messages
    pub fn can_edit_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_edit_messages, ..
            }) => can_edit_messages.unwrap_or_default(),
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can delete messages of other users.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_delete_messages`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_delete_messages`]: Administrator::can_delete_messages
    pub fn can_delete_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_delete_messages,
                ..
            }) => *can_delete_messages,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can manage voice chats.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_manage_voice_chats`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_manage_voice_chats`]: Administrator::can_manage_voice_chats
    pub fn can_manage_voice_chats(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_manage_voice_chats,
                ..
            }) => *can_manage_voice_chats,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can can invite new users to the chat.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the [`can_invite_users`]
    ///   privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_invite_users`]: Administrator::can_invite_users
    pub fn can_invite_users(&self) -> bool {
        match &self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_invite_users, ..
            }) => *can_invite_users,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can restrict, ban or unban chat members.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_restrict_members`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_restrict_members`]: Administrator::can_restrict_members
    pub fn can_restrict_members(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_restrict_members,
                ..
            }) => *can_restrict_members,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can pin messages, supergroups only.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat (even if the chat is not a supergroup)
    /// - is an administrator in the given chat and has the [`can_pin_messages`]
    ///   privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_pin_messages`]: Administrator::can_pin_messages
    pub fn can_pin_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_pin_messages, ..
            }) => can_pin_messages.unwrap_or_default(),
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can add new administrators with a subset of
    /// his own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed
    /// by the user).
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat (even if the chat is not a channel)
    /// - is an administrator in the given chat and has the
    ///   [`can_promote_members`] privilege.
    /// Returns `false` otherwise.
    ///
    /// [`can_promote_members`]: Administrator::can_promote_members
    pub fn can_promote_members(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator {
                can_promote_members,
                ..
            }) => *can_promote_members,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }
}

/// Methods for checking member rights.
impl ChatMemberKind {
    /// Returns `true` if the user can send text messages, contacts, locations
    /// and venues.
    ///
    /// I.e. returns **`false`** if the user
    /// - has left or has been banned in the chat
    /// - is restricted and doesn't have the [`can_send_messages`] right
    /// Returns `true` otherwise.
    ///
    /// [`can_send_messages`]: Restricted::can_send_messages
    pub fn can_send_messages(&self) -> bool {
        match &self {
            Self::Restricted(Restricted {
                can_send_messages, ..
            }) => *can_send_messages,
            Self::Owner(_) | Self::Administrator(_) | Self::Member => true,
            Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user is allowed to send audios, documents, photos,
    /// videos, video notes and voice notes.
    ///
    /// I.e. returns **`false`** if the user
    /// - has left or has been banned in the chat
    /// - is restricted and doesn't have the [`can_send_media_messages`] right
    /// Returns `true` otherwise.
    ///
    /// [`can_send_media_messages`]: Restricted::can_send_media_messages
    pub fn can_send_media_messages(&self) -> bool {
        match &self {
            Self::Restricted(Restricted {
                can_send_media_messages,
                ..
            }) => *can_send_media_messages,
            Self::Owner(_) | Self::Administrator(_) | Self::Member => true,
            Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user is allowed to send animations, games,
    /// stickers and use inline bots.
    ///
    /// I.e. returns **`false`** if the user
    /// - has left or has been banned from the chat
    /// - is restricted and doesn't have the [`can_send_media_messages`] right
    /// Returns `true` otherwise.
    ///
    /// [`can_send_media_messages`]: Restricted::can_send_media_messages
    pub fn can_send_other_messages(&self) -> bool {
        match &self {
            Self::Restricted(Restricted {
                can_send_other_messages,
                ..
            }) => *can_send_other_messages,
            Self::Owner(_) | Self::Administrator(_) | Self::Member => true,
            Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user is allowed to add web page previews to their
    /// messages.
    ///
    /// I.e. returns **`false`** if the user
    /// - has left or has been banned from the chat
    /// - is restricted and doesn't have the [`can_send_media_messages`] right
    /// Returns `true` otherwise.
    ///
    /// [`can_send_media_messages`]: Restricted::can_send_media_messages
    pub fn can_add_web_page_previews(&self) -> bool {
        match &self {
            Self::Restricted(Restricted {
                can_add_web_page_previews,
                ..
            }) => *can_add_web_page_previews,
            Self::Owner(_) | Self::Administrator(_) | Self::Member => true,
            Self::Left | Self::Banned(_) => false,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMemberStatus {
    Owner,
    Administrator,
    Member,
    Restricted,
    Left,
    Banned,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "user":{
                "id":1029940401,
                "is_bot":false,
                "first_name":"First",
                "last_name":"Last",
                "username":"fl",
                "language_code":"en"
            },
            "status":"administrator",
            "is_anonymous": false,
            "can_be_edited": false,
            "can_manage_chat": true,
            "can_change_info": true,
            "can_delete_messages": true,
            "can_manage_voice_chats": true,
            "can_invite_users": true,
            "can_restrict_members": true,
            "can_pin_messages": true,
            "can_promote_members": true
        }"#;
        let expected = ChatMember {
            user: User {
                id: 1029940401,
                is_bot: false,
                first_name: "First".to_string(),
                last_name: Some("Last".to_string()),
                username: Some("fl".to_string()),
                language_code: Some("en".to_string()),
            },
            kind: ChatMemberKind::Administrator(Administrator {
                custom_title: None,
                is_anonymous: false,
                can_be_edited: false,
                can_manage_chat: true,
                can_change_info: true,
                can_post_messages: None,
                can_edit_messages: None,
                can_delete_messages: true,
                can_manage_voice_chats: true,
                can_invite_users: true,
                can_restrict_members: true,
                can_pin_messages: Some(true),
                can_promote_members: true,
            }),
        };
        let actual = serde_json::from_str::<ChatMember>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
