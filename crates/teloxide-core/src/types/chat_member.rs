use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::types::{UntilDate, User};

/// This object contains information about one member of the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatmember).
#[serde_with_macros::skip_serializing_none]
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
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Owner {
    /// Custom title for this user.
    pub custom_title: Option<String>,

    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,
}

/// Administrator of the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[serde_with_macros::skip_serializing_none]
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
    #[serde(default)]
    pub can_post_messages: bool,

    /// `true` if the administrator can edit messages of other users and can pin
    /// messages, channels only.
    #[serde(default)]
    pub can_edit_messages: bool,

    /// `true` if the administrator can delete messages of other users.
    pub can_delete_messages: bool,

    /// `true` if the administrator can manage video chats.
    pub can_manage_video_chats: bool,

    /// `true` if the administrator can invite new users to the chat.
    pub can_invite_users: bool,

    /// `true` if the administrator can restrict, ban or unban chat members.
    pub can_restrict_members: bool,

    /// `true` if the administrator can pin messages, supergroups only.
    #[serde(default)]
    pub can_pin_messages: bool,

    /// `true`, if the user is allowed to create, rename, close, and reopen
    /// forum topics; supergroups only
    #[serde(default)]
    pub can_manage_topics: bool,

    /// `true` if the administrator can add new administrators with a subset of
    /// his own privileges or demote administrators that he has promoted,
    /// directly or indirectly (promoted by administrators that were appointed
    /// by the user).
    pub can_promote_members: bool,
}

/// User, restricted in the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Restricted {
    /// Date when restrictions will be lifted for this user.
    pub until_date: UntilDate,

    /// `true` if the user is a member of the chat at the moment of the request.
    pub is_member: bool,

    /// `true` if the user can send text messages, contacts, locations and
    /// venues.
    pub can_send_messages: bool,

    /// `true` if the user can send audios.
    pub can_send_audios: bool,

    /// `true` if the user can send documents.
    pub can_send_documents: bool,

    /// `true` if the user can send photos.
    pub can_send_photos: bool,

    /// `true` if the user can send videos.
    pub can_send_videos: bool,

    /// `true` if the user can send video notes.
    pub can_send_video_notes: bool,

    /// `true` if the user can send voice notes.
    pub can_send_voice_notes: bool,

    /// `true` if the user is allowed to send animations, games, stickers and
    /// use inline bots.
    pub can_send_other_messages: bool,

    /// `true` if the user is allowed to add web page previews to their
    /// messages.
    pub can_add_web_page_previews: bool,

    /// `true` if the user is allowed to change the chat title, photo
    /// and other settings.
    pub can_change_info: bool,

    /// `true` if the user is allowed to invite new users to the chat.
    pub can_invite_users: bool,

    /// `true` if the user is allowed to pin messages.
    pub can_pin_messages: bool,

    /// `true`, if the user is allowed to create, rename, close, and reopen
    /// forum topics
    pub can_manage_topics: bool,

    /// `true` if the user is allowed to send polls.
    pub can_send_polls: bool,
}

/// User that was banned in the chat and can't return to it or view chat
/// messages. This struct is part of the [`ChatMemberKind`] enum.
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Banned {
    /// Date when restrictions will be lifted for this user.
    pub until_date: UntilDate,
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
    #[must_use]
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
    #[must_use]
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
    #[must_use]
    pub fn is_administrator(&self) -> bool {
        matches!(self, Self::Administrator { .. })
    }

    /// Returns `true` if the user is a common [member] of the given chat.
    ///
    /// ⚠️ Don't confuse this with [`is_present`]. This method merely checks
    /// for [`ChatMemberKind::Member`] variant which is not enough to determine
    /// if the user is joinned to the chat. Use [`is_present`] for that instead.
    ///
    /// [member]: ChatMemberKind::Member
    /// [`is_present`]: ChatMemberKind::is_present
    #[must_use]
    pub fn is_member(&self) -> bool {
        matches!(self, Self::Member { .. })
    }

    /// Returns `true` if the user is [restricted] in the given chat.
    ///
    /// [restricted]: ChatMemberKind::Restricted
    #[must_use]
    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted { .. })
    }

    /// Returns `true` if the user [left] the given chat.
    ///
    /// ⚠️ Don't confuse this with [`is_present`]. This method merely checks
    /// for [`ChatMemberKind::Left`] variant which is not enough to determine
    /// if the user is joinned to the chat. Use [`is_present`] for that instead.
    ///
    /// [left]: ChatMemberKind::Left
    /// [`is_present`]: ChatMemberKind::is_present
    #[must_use]
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left { .. })
    }

    /// Returns `true` if the user is [banned] in the given chat.
    ///
    /// [banned]: ChatMemberKind::Banned
    #[must_use]
    pub fn is_banned(&self) -> bool {
        matches!(self, Self::Banned { .. })
    }
}

/// Compound methods for checking a user status.
impl ChatMemberKind {
    /// Returns `true` if the user is privileged in the given chat. i.e. if the
    /// user is either the [owner] or an [administrator] in the given chat.
    ///
    /// [owner]: ChatMemberKind::Owner
    /// [administrator]: ChatMemberKind::Administrator
    #[must_use]
    pub fn is_privileged(&self) -> bool {
        self.is_administrator() || self.is_owner()
    }

    /// Returns `true` if the user is currently present in the chat. i.e. if the
    /// user **hasn't** [left] or been [banned]. It also returns `false` if the
    /// user left the chat, but was [restricted].
    ///
    /// [left]: ChatMemberKind::Left
    /// [banned]: ChatMemberKind::Banned
    /// [restricted]: ChatMemberKind::Restricted
    #[must_use]
    pub fn is_present(&self) -> bool {
        let is_restricted_non_member =
            matches!(self, Self::Restricted(Restricted { is_member: false, .. }));

        !(self.is_left() || self.is_banned() || is_restricted_non_member)
    }
}

impl ChatMemberKind {
    /// Getter for [`Administrator::custom_title`] and [`Owner::custom_title`]
    /// fields.
    #[must_use]
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
    #[must_use]
    pub fn is_anonymous(&self) -> bool {
        match self {
            Self::Owner(Owner { is_anonymous, .. })
            | Self::Administrator(Administrator { is_anonymous, .. }) => *is_anonymous,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Getter for [`Restricted::until_date`] and [`Banned::until_date`] fields.
    #[must_use]
    pub fn until_date(&self) -> Option<UntilDate> {
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
    #[must_use]
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
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_manage_chat`]: Administrator::can_manage_chat
    #[must_use]
    pub fn can_manage_chat(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_manage_chat, .. }) => *can_manage_chat,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can post in the channel, channels only.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat (even if the chat is not a channel)
    /// - is an administrator in the given chat and has [`can_post_messages`]
    ///   privilege.
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_post_messages`]: Administrator::can_post_messages
    #[must_use]
    pub fn can_post_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_post_messages, .. }) => *can_post_messages,
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
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_edit_messages`]: Administrator::can_edit_messages
    #[must_use]
    pub fn can_edit_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_edit_messages, .. }) => *can_edit_messages,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can delete messages of other users.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_delete_messages`] privilege.
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_delete_messages`]: Administrator::can_delete_messages
    #[must_use]
    pub fn can_delete_messages(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_delete_messages, .. }) => *can_delete_messages,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can manage video chats.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_manage_video_chats`] privilege.
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_manage_video_chats`]: Administrator::can_manage_video_chats
    #[must_use]
    pub fn can_manage_video_chats(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_manage_video_chats, .. }) => {
                *can_manage_video_chats
            }
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }

    /// Returns `true` if the user can restrict, ban or unban chat members.
    ///
    /// I.e. returns `true` if the user
    /// - is the owner of the chat
    /// - is an administrator in the given chat and has the
    ///   [`can_restrict_members`] privilege.
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_restrict_members`]: Administrator::can_restrict_members
    #[must_use]
    pub fn can_restrict_members(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_restrict_members, .. }) => {
                *can_restrict_members
            }
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
    ///
    /// Returns `false` otherwise.
    ///
    /// [`can_promote_members`]: Administrator::can_promote_members
    #[must_use]
    pub fn can_promote_members(&self) -> bool {
        match self {
            Self::Owner(_) => true,
            Self::Administrator(Administrator { can_promote_members, .. }) => *can_promote_members,
            Self::Member | Self::Restricted(_) | Self::Left | Self::Banned(_) => false,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMemberStatus {
    #[serde(rename = "creator")]
    Owner,
    Administrator,
    Member,
    Restricted,
    Left,
    #[serde(rename = "kicked")]
    Banned,
}

/// Simple methods for checking a user status.
impl ChatMemberStatus {
    /// Returns `true` if the user is the [owner] of the given chat.
    ///
    /// [owner]: ChatMemberKind::Owner
    #[must_use]
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
    #[must_use]
    pub fn is_administrator(&self) -> bool {
        matches!(self, Self::Administrator { .. })
    }

    /// Returns `true` if the user is a common [member] of the given chat.
    ///
    /// ⚠️ Don't confuse this with [`is_present`]. This method merely checks
    /// for [`ChatMemberStatus::Member`] variant which is not enough to
    /// determine if the user is joinned to the chat. Use [`is_present`] for
    /// that instead.
    ///
    /// [member]: ChatMemberKind::Member
    /// [`is_present`]: ChatMemberKind::is_present
    #[must_use]
    pub fn is_member(&self) -> bool {
        matches!(self, Self::Member { .. })
    }

    /// Returns `true` if the user is [restricted] in the given chat.
    ///
    /// [restricted]: ChatMemberKind::Restricted
    #[must_use]
    pub fn is_restricted(&self) -> bool {
        matches!(self, Self::Restricted { .. })
    }

    /// Returns `true` if the user [left] the given chat.
    ///
    /// ⚠️ Don't confuse this with [`is_present`]. This method merely checks
    /// for [`ChatMemberStatus::Left`] variant which is not enough to determine
    /// if the user is joinned to the chat. Use [`is_present`] for that instead.
    ///
    /// [left]: ChatMemberKind::Left
    /// [`is_present`]: ChatMemberKind::is_present
    #[must_use]
    pub fn is_left(&self) -> bool {
        matches!(self, Self::Left { .. })
    }

    /// Returns `true` if the user is [banned] in the given chat.
    ///
    /// [banned]: ChatMemberKind::Banned
    #[must_use]
    pub fn is_banned(&self) -> bool {
        matches!(self, Self::Banned { .. })
    }
}

/// Compound methods for checking a user status.
impl ChatMemberStatus {
    /// Returns `true` if the user is privileged in the given chat. i.e. if the
    /// user is either the [owner] or an [administrator] in the given chat.
    ///
    /// [owner]: ChatMemberKind::Owner
    /// [administrator]: ChatMemberKind::Administrator
    #[must_use]
    pub fn is_privileged(&self) -> bool {
        self.is_administrator() || self.is_owner()
    }
}

#[cfg(test)]
mod tests {
    use crate::types::UserId;

    use super::*;

    #[test]
    fn deserialize_administrator() {
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
            "can_manage_video_chats": true,
            "can_invite_users": true,
            "can_restrict_members": true,
            "can_pin_messages": true,
            "can_promote_members": true
        }"#;
        let expected = ChatMember {
            user: User {
                id: UserId(1029940401),
                is_bot: false,
                first_name: "First".to_string(),
                last_name: Some("Last".to_string()),
                username: Some("fl".to_string()),
                language_code: Some("en".to_string()),
                is_premium: false,
                added_to_attachment_menu: false,
            },
            kind: ChatMemberKind::Administrator(Administrator {
                custom_title: None,
                is_anonymous: false,
                can_be_edited: false,
                can_manage_chat: true,
                can_change_info: true,
                can_post_messages: false,
                can_edit_messages: false,
                can_delete_messages: true,
                can_manage_video_chats: true,
                can_invite_users: true,
                can_restrict_members: true,
                can_pin_messages: true,
                can_promote_members: true,
                can_manage_topics: false,
            }),
        };
        let actual = serde_json::from_str::<ChatMember>(json).unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn deserialize_restricted() {
        let json = r#"{
            "user":{
                "id":1029940401,
                "is_bot":false,
                "first_name":"First",
                "last_name":"Last",
                "username":"fl",
                "language_code":"en"
            },
            "status":"restricted",
            "is_member": true,
            "can_send_messages": true,
            "can_send_media_messages": true,
            "can_send_audios": false,
            "can_send_documents": false,
            "can_send_photos": true,
            "can_send_videos": true,
            "can_send_video_notes": false,
            "can_send_voice_notes": true,
            "can_manage_topics": false,
            "can_send_polls": true,
            "can_send_other_messages": true,
            "can_add_web_page_previews": true,
            "can_change_info": true,
            "can_invite_users": true,
            "can_pin_messages": true,
            "until_date": 1620000000
        }"#;
        let expected = ChatMember {
            user: User {
                id: UserId(1029940401),
                is_bot: false,
                first_name: "First".to_string(),
                last_name: Some("Last".to_string()),
                username: Some("fl".to_string()),
                language_code: Some("en".to_string()),
                is_premium: false,
                added_to_attachment_menu: false,
            },
            kind: ChatMemberKind::Restricted(Restricted {
                is_member: true,
                can_send_messages: true,
                can_send_audios: false,
                can_send_documents: false,
                can_send_photos: true,
                can_send_videos: true,
                can_send_video_notes: false,
                can_send_voice_notes: true,
                can_manage_topics: false,
                can_send_polls: true,
                can_send_other_messages: true,
                can_add_web_page_previews: true,
                can_change_info: true,
                can_invite_users: true,
                can_pin_messages: true,
                until_date: UntilDate::Date(
                    chrono::DateTime::from_timestamp(1620000000, 0).unwrap(),
                ),
            }),
        };
        let actual = serde_json::from_str::<ChatMember>(json).unwrap();
        assert_eq!(actual, expected)
    }
}
