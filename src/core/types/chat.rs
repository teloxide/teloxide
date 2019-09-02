use serde::Deserialize;
use crate::core::types::{ChatPhoto, ChatPermissions, Message};

#[derive(Debug, Deserialize)]
pub struct Chat {
    id: i64,
    chat_type: String,
    title: Option<String>,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Box<Message>>,
    permissions: Option<ChatPermissions>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<bool>,
}
