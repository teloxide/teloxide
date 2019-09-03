use std::future::Future;

use crate::core::network::ResponseResult;

use serde::de::DeserializeOwned;
use reqwest::r#async::Client;


mod form_builder;


/// Request that can be sent to telegram.
/// `ReturnValue` - a type that will be returned from Telegram.
pub trait Request {
    type ReturnValue: DeserializeOwned;

    /// Send request to telegram
    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>>;
}

pub type RequestFuture<T> = Box<dyn Future<Output = T>>;

// todo: better name?
#[derive(Debug)]
pub(crate) struct RequestInfo {
    pub(crate) client: Client,
    pub(crate) token: String,
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
