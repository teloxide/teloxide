use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ChatPermissions {
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_polls: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
    can_change_info: Option<bool>,
    can_invite_users: Option<bool>,
    can_pin_messages: Option<bool>,
}
