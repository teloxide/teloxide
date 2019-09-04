use std::future::Future;
use std::pin::Pin;

use reqwest::r#async::Client;
use reqwest::StatusCode;
use serde::de::DeserializeOwned;

mod form_builder;

#[derive(Debug, Display)]
pub enum RequestError {
    #[display(fmt = "Telegram error #{}: {}", status_code, description)]
    ApiError {
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

// todo: better name?
#[derive(Debug)]
pub struct RequestInfo<'a> {
    pub client: &'a Client,
    pub token: &'a str,
}

/// Unique identifier for the target chat or username of the target channel (in
/// the format @channelusername)
#[derive(Debug, Display, Serialize, From, PartialEq, Eq)]
pub enum ChatId {
    /// chat identifier
    #[display(fmt = "{}", _0)]
    Id(i64),
    /// _channel_ username (in the format @channelusername)
    #[display(fmt = "{}", _0)]
    ChannelUsername(String),
}

pub mod get_me;
pub mod send_message;
