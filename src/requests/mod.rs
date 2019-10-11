use async_trait::async_trait;
use reqwest::r#async::Client;
use serde::de::DeserializeOwned;

use crate::RequestError;

pub use self::{
    answer_pre_checkout_query::AnswerPreCheckoutQuery,
    answer_shipping_query::AnswerShippingQuery,
    edit_message_live_location::EditMessageLiveLocation,
    forward_message::ForwardMessage,
    get_chat::GetChat,
    get_file::GetFile,
    get_me::GetMe,
    get_updates::GetUpdates,
    get_user_profile_photos::GetUserProfilePhotos,
    kick_chat_member::KickChatMember,
    pin_chat_message::PinChatMessage,
    promote_chat_member::PromoteChatMember,
    restrict_chat_member::RestrictChatMember,
    send_animation::SendAnimation,
    send_audio::SendAudio,
    send_chat_action::SendChatAction,
    send_contact::SendContact,
    send_document::SendDocument,
    send_location::SendLocation,
    send_media_group::SendMediaGroup,
    send_message::SendMessage,
    send_photo::SendPhoto,
    send_poll::SendPoll,
    send_venue::SendVenue,
    send_video::SendVideo,
    send_video_note::SendVideoNote,
    send_voice::SendVoice,
    stop_message_live_location::StopMessageLiveLocation,
    unban_chat_member::UnbanChatMember,
    unpin_chat_message::UnpinChatMessage,
};

mod form_builder;
mod utils;

pub type ResponseResult<T> = Result<T, RequestError>;

/// Request that can be sent to telegram.
/// `ReturnValue` - a type that will be returned from Telegram.
#[async_trait]
pub trait Request {
    type ReturnValue: DeserializeOwned;

    /// Send request to telegram
    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue>;
}

#[derive(Debug, Clone)]
pub struct RequestContext<'a> {
    pub client: &'a Client,
    pub token: &'a str,
}

mod answer_pre_checkout_query;
mod answer_shipping_query;
mod edit_message_live_location;
mod forward_message;
mod get_chat;
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
mod stop_message_live_location;
mod unban_chat_member;
mod unpin_chat_message;
