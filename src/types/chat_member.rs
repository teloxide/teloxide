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
    Creator {
        /// Custom title for this user.
        custom_title: Option<String>,
    },
    Administrator {
        /// Custom title for this user.
        custom_title: Option<String>,

        /// `true`, if the bot is allowed to edit
        /// administrator privileges of that user.
        can_be_edited: bool,

        /// `true`, if the administrator can change the chat
        /// title, photo and other settings.
        can_change_info: bool,

        /// `true`, if the administrator can post in the
        /// channel, channels only.
        can_post_messages: Option<bool>,

        /// `true`, if the administrator can edit messages of
        /// other users and can pin messages, channels only.
        can_edit_messages: Option<bool>,

        /// `true`, if the administrator can delete messages
        /// of other users.
        can_delete_messages: bool,

        /// `true`, if the administrator can invite new users
        /// to the chat.
        can_invite_users: bool,

        /// `true`, if the administrator can restrict,
        /// ban or unban chat members.
        can_restrict_members: bool,

        /// `true`, if the administrator can pin messages,
        /// supergroups only.
        can_pin_messages: Option<bool>,

        /// `true`, if the administrator can add new
        /// administrators with a subset of his own privileges or demote
        /// administrators that he has promoted, directly or indirectly
        /// (promoted by administrators that were appointed by the
        /// user).
        can_promote_members: bool,
    },
    Member,
    Restricted {
        /// Date when restrictions will be lifted for
        /// this user, unix time.
        until_date: i32,

        /// Restricted only. `true`, if the user can send text messages,
        /// contacts, locations and venues.
        can_send_messages: bool,

        /// Restricted only. `true`, if the user is allowed to send audios,
        /// documents, photos, videos, video notes and voice notes.
        can_send_media_messages: bool,

        /// Restricted only. `true`, if the user is allowed to send animations,
        /// games, stickers and use inline bots.
        can_send_other_messages: bool,

        /// Restricted only. `true`, if the user is allowed to add web page
        /// previews to their messages.
        can_add_web_page_previews: bool,
    },
    Left,
    Kicked {
        /// Date when restrictions will be lifted for
        /// this user, unix time.
        until_date: i32,
    },
}

impl ChatMember {
    pub fn status(&self) -> ChatMemberStatus {
        match &self.kind {
            ChatMemberKind::Creator { .. } => ChatMemberStatus::Creator,
            ChatMemberKind::Administrator { .. } => ChatMemberStatus::Administrator,
            ChatMemberKind::Member => ChatMemberStatus::Member,
            ChatMemberKind::Restricted { .. } => ChatMemberStatus::Restricted,
            ChatMemberKind::Left => ChatMemberStatus::Left,
            ChatMemberKind::Kicked { .. } => ChatMemberStatus::Kicked,
        }
    }
}
impl ChatMemberKind {
    pub fn custom_title(&self) -> Option<&str> {
        match &self {
            Self::Administrator { custom_title, .. } | Self::Creator { custom_title, .. } => {
                //Some(custom_title.as_str())
                custom_title.as_deref()
            }
            Self::Member | Self::Restricted { .. } | Self::Left | Self::Kicked { .. } => None,
        }
    }

    pub fn until_date(&self) -> Option<i32> {
        match &self {
            Self::Creator { .. } | Self::Administrator { .. } | Self::Member | Self::Left => None,
            Self::Restricted { until_date, .. } | Self::Kicked { until_date, .. } => {
                Some(*until_date)
            }
        }
    }

    pub fn can_be_edited(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_be_edited, .. } => Some(*can_be_edited),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_change_info(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_change_info, .. } => Some(*can_change_info),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_post_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_post_messages, .. } => *can_post_messages,
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_edit_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_edit_messages, .. } => *can_edit_messages,
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_delete_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_delete_messages, .. } => Some(*can_delete_messages),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_invite_users(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_invite_users, .. } => Some(*can_invite_users),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_restrict_members(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_restrict_members, .. } => Some(*can_restrict_members),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_pin_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_pin_messages, .. } => *can_pin_messages,
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_promote_members(&self) -> Option<bool> {
        match &self {
            Self::Administrator { can_promote_members, .. } => Some(*can_promote_members),
            Self::Creator { .. }
            | Self::Member
            | Self::Restricted { .. }
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_send_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted { can_send_messages, .. } => Some(*can_send_messages),
            Self::Creator { .. }
            | Self::Administrator { .. }
            | Self::Member
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_send_media_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted { can_send_media_messages, .. } => Some(*can_send_media_messages),
            Self::Creator { .. }
            | Self::Administrator { .. }
            | Self::Member
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_send_other_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted { can_send_other_messages, .. } => Some(*can_send_other_messages),
            Self::Creator { .. }
            | Self::Administrator { .. }
            | Self::Member
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }

    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match &self {
            Self::Restricted { can_add_web_page_previews, .. } => Some(*can_add_web_page_previews),
            Self::Creator { .. }
            | Self::Administrator { .. }
            | Self::Member
            | Self::Left
            | Self::Kicked { .. } => None,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Restricted,
    Left,
    Kicked,
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
            "can_be_edited":false,
            "can_change_info":true,
            "can_delete_messages":true,
            "can_invite_users":true,
            "can_restrict_members":true,
            "can_pin_messages":true,
            "can_promote_members":true
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
            kind: ChatMemberKind::Administrator {
                custom_title: None,
                can_be_edited: false,
                can_change_info: true,
                can_post_messages: None,
                can_edit_messages: None,
                can_delete_messages: true,
                can_invite_users: true,
                can_restrict_members: true,
                can_pin_messages: Some(true),
                can_promote_members: true,
            },
        };
        let actual = serde_json::from_str::<ChatMember>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
