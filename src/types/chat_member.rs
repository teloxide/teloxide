use serde::{Deserialize, Serialize};

use crate::types::User;

// TODO: ChatMemberKind?...
/// This object contains information about one member of the chat.
///
/// [The official docs](https://core.telegram.org/bots/api#chatmember).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct ChatMember {
    /// Information about the user.
    pub user: User,

    /// The member's status in the chat.
    pub status: ChatMemberStatus,

    /// Owner and administrators only. Custom title for this user
    pub custom_title: Option<String>,

    /// Restricted and kicked only. Date when restrictions will be lifted for
    /// this user, unix time.
    pub until_date: Option<i32>,

    /// Administrators only. `true`, if the bot is allowed to edit
    /// administrator privileges of that user.
    pub can_be_edited: Option<bool>,

    /// Administrators only. `true`, if the administrator can change the chat
    /// title, photo and other settings.
    pub can_change_info: Option<bool>,

    /// Administrators only. `true`, if the administrator can post in the
    /// channel, channels only.
    pub can_post_messages: Option<bool>,

    /// Administrators only. `true`, if the administrator can edit messages of
    /// other users and can pin messages, channels only.
    pub can_edit_messages: Option<bool>,

    /// Administrators only. `true`, if the administrator can delete messages
    /// of other users.
    pub can_delete_messages: Option<bool>,

    /// Administrators only. `true`, if the administrator can invite new users
    /// to the chat.
    pub can_invite_users: Option<bool>,

    /// Administrators only. `true`, if the administrator can restrict,
    /// ban or unban chat members.
    pub can_restrict_members: Option<bool>,

    /// Administrators only. `true`, if the administrator can pin messages,
    /// supergroups only.
    pub can_pin_messages: Option<bool>,

    /// Administrators only. `true`, if the administrator can add new
    /// administrators with a subset of his own privileges or demote
    /// administrators that he has promoted, directly or indirectly (promoted
    /// by administrators that were appointed by the user).
    pub can_promote_members: Option<bool>,

    /// Restricted only. `true`, if the user can send text messages,
    /// contacts, locations and venues.
    pub can_send_messages: Option<bool>,

    /// Restricted only. `true`, if the user is allowed to send audios,
    /// documents, photos, videos, video notes and voice notes.
    pub can_send_media_messages: Option<bool>,

    /// Restricted only. `true`, if the user is allowed to send animations,
    /// games, stickers and use inline bots.
    pub can_send_other_messages: Option<bool>,

    /// Restricted only. `true`, if the user is allowed to add web page
    /// previews to their messages.
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
                "id":12345,
                "is_bot":false,
                "first_name":"firstName"
            },
            "status":"creator",
            "until_date":123456,
            "can_be_edited":true,
            "can_post_messages":true,
            "can_edit_messages":true,
            "can_delete_messages":true,
            "can_restrict_members":true,
            "can_promote_members":true,
            "can_change_info":true,
            "can_invite_users":true,
            "can_pin_messages":true,
            "is_member":true,
            "can_send_messages":true,
            "can_send_media_messages":true,
            "can_send_polls":true,
            "can_send_other_messages":true,
            "can_add_web_page_previews":true
        }"#;
        let expected = ChatMember {
            user: User {
                id: 12345,
                is_bot: false,
                first_name: "firstName".to_string(),
                last_name: None,
                username: None,
                language_code: None,
            },
            status: ChatMemberStatus::Creator,
            custom_title: None,
            until_date: Some(123_456),
            can_be_edited: Some(true),
            can_change_info: Some(true),
            can_post_messages: Some(true),
            can_edit_messages: Some(true),
            can_delete_messages: Some(true),
            can_invite_users: Some(true),
            can_restrict_members: Some(true),
            can_pin_messages: Some(true),
            can_promote_members: Some(true),
            can_send_messages: Some(true),
            can_send_media_messages: Some(true),
            can_send_other_messages: Some(true),
            can_add_web_page_previews: Some(true),
        };
        let actual = serde_json::from_str::<ChatMember>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
