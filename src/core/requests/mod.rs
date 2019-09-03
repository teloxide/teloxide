use reqwest::r#async::multipart::Form;
use serde::de::DeserializeOwned;

mod form_builder;

/// Request that can be sended to telegram.
/// `ReturnValue` - a type that will be returned from Telegram.
pub trait Request {
    type ReturnValue: DeserializeOwned;

    /// Get name of the request (e.g. "getMe" or "sendMessage")
    fn name(&self) -> &str;

    /// Form with params
    fn params(self) -> Option<Form>;

    /// Bot token
    fn token(&self) -> &str;
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
