//! Raw API functions.

use async_trait::async_trait;
use serde::de::DeserializeOwned;

pub use answer_callback_query::*;
pub use answer_pre_checkout_query::*;
pub use answer_shipping_query::*;
pub use delete_chat_sticker_set::*;
pub use edit_message_live_location::*;
pub use forward_message::*;
pub use get_chat::*;
pub use get_chat_member::*;
pub use get_chat_members_count::*;
pub use get_file::*;
pub use get_me::*;
pub use get_updates::*;
pub use get_user_profile_photos::*;
pub use kick_chat_member::*;
pub use pin_chat_message::*;
pub use promote_chat_member::*;
pub use restrict_chat_member::*;
pub use send_animation::*;
pub use send_audio::*;
pub use send_chat_action::*;
pub use send_contact::*;
pub use send_document::*;
pub use send_location::*;
pub use send_media_group::*;
pub use send_message::*;
pub use send_photo::*;
pub use send_poll::*;
pub use send_venue::*;
pub use send_video::*;
pub use send_video_note::*;
pub use send_voice::*;
pub use set_chat_description::*;
pub use set_chat_sticker_set::*;
pub use stop_message_live_location::*;
pub use unban_chat_member::*;
pub use unpin_chat_message::*;

mod form_builder;
mod utils;

mod answer_callback_query;
mod answer_pre_checkout_query;
mod answer_shipping_query;
mod delete_chat_sticker_set;
mod edit_message_live_location;
mod forward_message;
mod get_chat;
mod get_chat_member;
mod get_chat_members_count;
mod get_file;
mod get_me;
mod get_updates;
mod get_user_profile_photos;
mod kick_chat_member;
mod pin_chat_message;
mod promote_chat_member;
mod restrict_chat_member;
mod send_animation;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_document;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_poll;
mod send_venue;
mod send_video;
mod send_video_note;
mod send_voice;
mod set_chat_description;
mod set_chat_sticker_set;
mod stop_message_live_location;
mod unban_chat_member;
mod unpin_chat_message;

/// A type that is returned from `Request::send_boxed`.
pub type ResponseResult<T> = Result<T, crate::RequestError>;

/// A request that can be sent to Telegram.
#[async_trait]
pub trait Request {
    /// A type of response.
    type Output: DeserializeOwned;

    /// Send this request.
    async fn send_boxed(self) -> ResponseResult<Self::Output>;
}
