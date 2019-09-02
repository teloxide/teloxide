use serde::Deserialize;

use crate::core::types::{ChatPermissions, ChatPhoto, Message};

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct Chat {
    pub id: i64,
    pub chat_type: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub photo: Option<ChatPhoto>,
    pub description: Option<String>,
    pub invite_link: Option<String>,
    pub pinned_message: Option<Box<Message>>,
    pub permissions: Option<ChatPermissions>,
    pub sticker_set_name: Option<String>,
    pub can_set_sticker_set: Option<bool>,
}
