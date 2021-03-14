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
    Creator(Creator),
    Administrator(Administrator),
    Member,
    Restricted(Restricted),
    Left,
    Kicked(Kicked),
}

/// Creator of the group. This struct is part of the [`ChatMemberKind`] enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Creator {
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

    /// True, if the user's presence in the chat is hidden
    pub is_anonymous: bool,

    /// `true`, if the bot is allowed to edit
    /// administrator privileges of that user.
    pub can_be_edited: bool,

    /// `true`, if the administrator can change the chat
    /// title, photo and other settings.
    pub can_change_info: bool,

    /// `true`, if the administrator can post in the
    /// channel, channels only.
    pub can_post_messages: Option<bool>,

    /// `true`, if the administrator can edit messages of
    /// other users and can pin messages, channels only.
    pub can_edit_messages: Option<bool>,

    /// `true`, if the administrator can delete messages
    /// of other users.
    pub can_delete_messages: bool,

    /// `true`, if the administrator can invite new users
    /// to the chat.
    pub can_invite_users: bool,

    /// `true`, if the administrator can restrict,
    /// ban or unban chat members.
    pub can_restrict_members: bool,

    /// `true`, if the administrator can pin messages,
    /// supergroups only.
    pub can_pin_messages: Option<bool>,

    /// `true`, if the administrator can add new
    /// administrators with a subset of his own privileges or demote
    /// administrators that he has promoted, directly or indirectly
    /// (promoted by administrators that were appointed by the
    /// user).
    pub can_promote_members: bool,
}

/// User, restricted in the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Restricted {
    /// Date when restrictions will be lifted for
    /// this user, unix time.
    pub until_date: i32,

    /// Restricted only. `true`, if the user can send text messages,
    /// contacts, locations and venues.
    pub can_send_messages: bool,

    /// Restricted only. `true`, if the user is allowed to send audios,
    /// documents, photos, videos, video notes and voice notes.
    pub can_send_media_messages: bool,

    /// Restricted only. `true`, if the user is allowed to send animations,
    /// games, stickers and use inline bots.
    pub can_send_other_messages: bool,

    /// Restricted only. `true`, if the user is allowed to add web page
    /// previews to their messages.
    pub can_add_web_page_previews: bool,
}

/// User kicked from the group. This struct is part of the [`ChatMemberKind`]
/// enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Kicked {
    /// Date when restrictions will be lifted for
    /// this user, unix time.
    pub until_date: i32,
}

impl ChatMember {
    pub fn status(&self) -> ChatMemberStatus {
        match &self.kind {
            ChatMemberKind::Creator(_) => ChatMemberStatus::Creator,
            ChatMemberKind::Administrator(_) => ChatMemberStatus::Administrator,
            ChatMemberKind::Member => ChatMemberStatus::Member,
            ChatMemberKind::Restricted(_) => ChatMemberStatus::Restricted,
            ChatMemberKind::Left => ChatMemberStatus::Left,
            ChatMemberKind::Kicked(_) => ChatMemberStatus::Kicked,
        }
    }
}

impl ChatMemberKind {
    /// Getter for [`Administrator::custom_title`] and [`Creator::custom_title`]
    /// fields.
    pub fn custom_title(&self) -> Option<&str> {
        match &self {
            Self::Administrator(Administrator { custom_title, .. })
            | Self::Creator(Creator { custom_title, .. }) => custom_title.as_deref(),
            Self::Member | Self::Restricted(_) | Self::Left | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Restricted::until_date`] and [`Kicked::until_date`] fields.
    pub fn until_date(&self) -> Option<i32> {
        match &self {
            Self::Creator(_) | Self::Administrator(_) | Self::Member | Self::Left => None,
            Self::Restricted(Restricted { until_date, .. })
            | Self::Kicked(Kicked { until_date, .. }) => Some(*until_date),
        }
    }

    /// Getter for [`Administrator::can_be_edited`] field.
    pub fn can_be_edited(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator { can_be_edited, .. }) => Some(*can_be_edited),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_change_info`] field.
    pub fn can_change_info(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_change_info, ..
            }) => Some(*can_change_info),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_post_messages`] field.
    pub fn can_post_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_post_messages, ..
            }) => *can_post_messages,
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_edit_messages`] field.
    pub fn can_edit_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_edit_messages, ..
            }) => *can_edit_messages,
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_delete_messages`] field.
    pub fn can_delete_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_delete_messages,
                ..
            }) => Some(*can_delete_messages),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_invite_users`] field.
    pub fn can_invite_users(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_invite_users, ..
            }) => Some(*can_invite_users),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_restrict_members`] field.
    pub fn can_restrict_members(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_restrict_members,
                ..
            }) => Some(*can_restrict_members),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_pin_messages`] field.
    pub fn can_pin_messages(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_pin_messages, ..
            }) => *can_pin_messages,
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Administrator::can_promote_members`] field.
    pub fn can_promote_members(&self) -> Option<bool> {
        match &self {
            Self::Administrator(Administrator {
                can_promote_members,
                ..
            }) => Some(*can_promote_members),
            Self::Creator(_)
            | Self::Member
            | Self::Restricted(_)
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Restricted::can_send_messages`] field.
    pub fn can_send_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted(Restricted {
                can_send_messages, ..
            }) => Some(*can_send_messages),
            Self::Creator(_)
            | Self::Administrator(_)
            | Self::Member
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Restricted::can_send_media_messages`] field.
    pub fn can_send_media_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted(Restricted {
                can_send_media_messages,
                ..
            }) => Some(*can_send_media_messages),
            Self::Creator(_)
            | Self::Administrator(_)
            | Self::Member
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Restricted::can_send_other_messages`] field.
    pub fn can_send_other_messages(&self) -> Option<bool> {
        match &self {
            Self::Restricted(Restricted {
                can_send_other_messages,
                ..
            }) => Some(*can_send_other_messages),
            Self::Creator(_)
            | Self::Administrator(_)
            | Self::Member
            | Self::Left
            | Self::Kicked(_) => None,
        }
    }

    /// Getter for [`Restricted::can_add_web_page_previews`] field.
    pub fn can_add_web_page_previews(&self) -> Option<bool> {
        match &self {
            Self::Restricted(Restricted {
                can_add_web_page_previews,
                ..
            }) => Some(*can_add_web_page_previews),
            Self::Creator(_)
            | Self::Administrator(_)
            | Self::Member
            | Self::Left
            | Self::Kicked(_) => None,
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
            "can_promote_members":true,
            "is_anonymous":false,
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
                can_be_edited: false,
                can_change_info: true,
                can_post_messages: None,
                can_edit_messages: None,
                can_delete_messages: true,
                can_invite_users: true,
                can_restrict_members: true,
                can_pin_messages: Some(true),
                can_promote_members: true,
                is_anonymous: false,
            }),
        };
        let actual = serde_json::from_str::<ChatMember>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
