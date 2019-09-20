use crate::types::User;

/// This object contains information about one member of the chat.
#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
pub struct ChatMember {
    /// Information about the user.
    pub user: User,
    /// The member's status in the chat.
    pub status: ChatMemberStatus,
    ///Optional. Restricted and kicked only. Date when restrictions will be
    /// lifted for this user, unix time
    pub until_date: Option<i32>,
    ///Optional. Administrators only. True, if the bot is allowed to edit
    /// administrator privileges of that user
    pub can_be_edited: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can change
    /// the chat title, photo and other settings
    pub can_change_info: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can post in
    /// the channel, channels only
    pub can_post_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can edit
    /// messages of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can delete
    /// messages of other users
    pub can_delete_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can invite
    /// new users to the chat
    pub can_invite_users: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can restrict,
    /// ban or unban chat members
    pub can_restrict_members: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can pin
    /// messages, supergroups only
    pub can_pin_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can add new
    /// administrators with a subset of his own privileges or demote
    /// administrators that he has promoted, directly or indirectly (promoted
    /// by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    ///Optional. Restricted only. True, if the user can send text messages,
    /// contacts, locations and venues
    pub can_send_messages: Option<bool>,
    ///Optional. Restricted only. True, if the user can send audios,
    /// documents, photos, videos, video notes and voice notes, implies
    /// can_send_messages
    pub can_send_media_messages: Option<bool>,
    ///Optional. Restricted only. True, if the user can send animations,
    /// games, stickers and use inline bots, implies
    /// can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    ///Optional. Restricted only. True, if user may add web page previews to
    /// his messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}

#[derive(Deserialize, Debug, Hash, PartialEq, Eq, Clone)]
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
            until_date: Some(123456),
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
