use reqwest::r#async::multipart::Form;


/// Request that can be sended to telegram.
/// `R` - return type.
pub trait Request<R: serde::de::DeserializeOwned> {
    /// Get name of the request (e.g. "getMe" or "sendMessage")
    fn name(&self) -> &str;

    /// Form with params
    fn params(self) -> Option<Form>;

    /// Bot token
    fn token(&self) -> &str;
}

/// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
#[derive(Debug, Serialize, From)]
pub enum ChatId {
    /// chat identifier
    Id(i32), // 32?
    /// _channel_ username (in the format @channelusername)
    ChannelUsername(String)
}

pub mod get_me;
pub mod send_message;