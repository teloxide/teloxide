use crate::core::payments::{Invoice, SuccessfulPayment};
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

#[derive(Debug, Deserialize)]
pub struct Message {
    message_id: i64,
    from: Option<Box<User>>,
    date: i64,
    chat: Chat,
    forward_from: Option<User>,
    forward_from_chat: Option<Chat>,
    forward_from_message_id: Option<i64>,
    forward_signature: Option<String>,
    forward_sender_name: Option<String>,
    forward_date: Option<i64>,
    reply_to_message: Option<Box<Message>>,
    edit_date: Option<i64>,
    media_group_id: Option<String>,
    author_signature: Option<String>,
    text: Option<String>,
    entities: Option<Vec<MessageEntity>>,
    caption_entities: Option<Vec<MessageEntity>>,
    audio: Option<Audio>,
    document: Option<Document>,
    animation: Option<Animation>,
    game: Option<Game>,
    photo: Option<Vec<PhotoSize>>,
    sticker: Option<Stickers>,
    video: Option<Video>,
    voice: Option<Voice>,
    video_note: Option<VideoNote>,
    caption: Option<String>,
    contact: Option<Contact>,
    location: Option<Location>,
    venue: Option<Venue>,
    poll: Option<Poll>,
    new_chat_members: Option<Vec<User>>,
    left_chat_member: Option<User>,
    new_chat_title: Option<String>,
    new_chat_photo: Option<Vec<PhotoSize>>,
    delete_chat_photo: Option<bool>,
    group_chat_created: Option<bool>,
    supergroup_chat_created: Option<bool>,
    channel_chat_created: Option<bool>,
    migrate_to_chat_id: Option<i64>,
    migrate_from_chat_id: Option<i64>,
    pinned_message: Option<Box<Message>>,
    invoice: Option<Invoice>,
    successful_payment: Option<SuccessfulPayment>,
    connected_website: Option<String>,
    passport_data: Option<PassportData>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[derive(Debug, Deserialize)]
pub struct ChatPhoto {
    small_file_id: String,
    big_file_id: String,
}

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

#[derive(Debug, Deserialize)]
pub struct Document {
    file_id: String,
    thumb: Option<PhotoSize>,
    file_name: Option<String>,
    mime_type: Option<String>,
    file_size: Option<i64>,
}
