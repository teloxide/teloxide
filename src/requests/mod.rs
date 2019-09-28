mod form_builder;
mod utils;

use async_trait::async_trait;
use reqwest::r#async::Client;
use serde::de::DeserializeOwned;

use crate::RequestError;

pub use self::{
    answer_pre_checkout_query::AnswerPreCheckoutQuery,
    answer_shipping_query::AnswerShippingQuery,
    edit_message_live_location::EditMessageLiveLocation,
    forward_message::ForwardMessage, get_chat::GetChat, get_file::GetFile,
    get_me::GetMe, get_updates::GetUpdates,
    get_user_profile_photos::GetUserProfilePhotos,
    kick_chat_member::KickChatMember, pin_chat_message::PinChatMessage,
    restrict_chat_member::RestrictChatMember,
    send_audio::SendAudio, send_chat_action::SendChatAction,
    send_contact::SendContact, send_location::SendLocation,
    send_media_group::SendMediaGroup, send_message::SendMessage,
    send_photo::SendPhoto, send_poll::SendPoll, send_venue::SendVenue,
    stop_message_live_location::StopMessageLiveLocation,
    unban_chat_member::UnbanChatMember,
};

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

/// Unique identifier for the target chat or username of the target channel (in
/// the format @channelusername)
#[derive(Debug, Display, Serialize, From, PartialEq, Eq, Clone)]
#[serde(untagged)]
pub enum ChatId {
    /// chat identifier
    #[display(fmt = "{}", _0)]
    Id(i64),
    /// _channel_ username (in the format @channelusername)
    #[display(fmt = "{}", _0)]
    ChannelUsername(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_id_id_serialization() {
        let expected_json = String::from(r#"123456"#);
        let actual_json = serde_json::to_string(&ChatId::Id(123456)).unwrap();

        assert_eq!(expected_json, actual_json)
    }

    #[test]
    fn chat_id_channel_username_serialization() {
        let expected_json = String::from(r#""@username""#);
        let actual_json = serde_json::to_string(&ChatId::ChannelUsername(
            String::from("@username"),
        ))
        .unwrap();

        assert_eq!(expected_json, actual_json)
    }
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
mod restrict_chat_member;
mod send_audio;
mod send_chat_action;
mod send_contact;
mod send_location;
mod send_media_group;
mod send_message;
mod send_photo;
mod send_poll;
mod send_venue;
mod stop_message_live_location;
mod unban_chat_member;
