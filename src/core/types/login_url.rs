use serde::Deserialize;

#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct LoginUrl {
    url: String,
    forward_text: Option<String>,
    bot_username: Option<String>,
    request_write_access: Option<bool>,
}