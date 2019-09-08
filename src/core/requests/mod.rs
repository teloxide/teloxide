use std::pin::Pin;
use std::future::Future;

use reqwest::{
    r#async::Client, StatusCode
};
use serde::de::DeserializeOwned;


mod form_builder;
mod utils;

#[derive(Debug, Display)]
pub enum RequestError {
    #[display(fmt = "Telegram error #{}: {}", status_code, description)]
    ApiError { // TODO: add response parameters
        status_code: StatusCode,
        description: String,
    },

    #[display(fmt = "Network error: {err}", err = _0)]
    NetworkError(reqwest::Error),

    #[display(fmt = "InvalidJson error caused by: {err}", err = _0)]
    InvalidJson(serde_json::Error),
}

impl std::error::Error for RequestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RequestError::ApiError { .. } => None,
            RequestError::NetworkError(err) => Some(err),
            RequestError::InvalidJson(err) => Some(err),
        }
    }
}

pub type ResponseResult<T> = Result<T, RequestError>;

/// Request that can be sent to telegram.
/// `ReturnValue` - a type that will be returned from Telegram.
pub trait Request<'a> {
    type ReturnValue: DeserializeOwned;

    /// Send request to telegram
    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>>;
}

pub type RequestFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

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
        let actual_json = serde_json::to_string(&ChatId::ChannelUsername(String::from("@username"))).unwrap();

        assert_eq!(expected_json, actual_json)
    }
}

pub mod get_me;
pub mod send_message;
pub mod forward_message;
pub mod send_photo;
pub mod send_media_group;
pub mod send_audio;
