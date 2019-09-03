#[derive(Debug, Deserialize, Hash, PartialEq, Eq)]
pub struct LoginUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}