use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct User {
    id: i64,
    is_bot: bool,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
    language_code: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    id: i64,
    chat_type: String,
    title: Option<String>,
    username:Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    photo: Option<ChatPhoto>,
    description: Option<String>,
    invite_link: Option<String>,
    pinned_message: Option<Message>,
    permissions: Option<ChatPermissions>,
    sticker_set_name: Option<String>,
    can_set_sticker_set: Option<Bool>,
}