#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Serialize, Clone)]
pub struct ChatPermissions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_media_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_polls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_send_other_messages: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_add_web_page_previews: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_change_info: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
}
