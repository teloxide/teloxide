mod form_builder;
mod utils;

use async_trait::async_trait;
use reqwest::r#async::Client;
use serde::de::DeserializeOwned;

use crate::RequestError;

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

pub mod answer_pre_checkout_query;
pub mod answer_shipping_query;
pub mod edit_message_live_location;
pub mod forward_message;
pub mod get_chat;
pub mod get_file;
pub mod get_me;
pub mod get_updates;
pub mod get_user_profile_photos;
pub mod kick_chat_member;
pub mod restrict_chat_member;
pub mod send_audio;
pub mod send_chat_action;
pub mod send_contact;
pub mod send_location;
pub mod send_media_group;
pub mod send_message;
pub mod send_photo;
pub mod send_poll;
pub mod send_venue;
pub mod stop_message_live_location;
pub mod unban_chat_member;
