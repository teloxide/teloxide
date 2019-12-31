use serde::{Deserialize, Serialize};

/// This object represents a Telegram user or bot.
///
/// [The official docs](https://core.telegram.org/bots/api#user).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: i32,

    /// `true`, if this user is a bot.
    pub is_bot: bool,

    /// User‘s or bot’s first name.
    pub first_name: String,

    /// User‘s or bot’s last name.
    pub last_name: Option<String>,

    /// User‘s or bot’s username.
    pub username: Option<String>,

    /// [IETF language tag] of the user's language.
    ///
    /// [IETF language tag]: https://en.wikipedia.org/wiki/IETF_language_tag
    pub language_code: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":12345,
            "is_bot":false,
            "first_name":"firstName",
            "last_name":"lastName",
            "username":"Username",
            "language_code":"languageCode"
        }"#;
        let expected = User {
            id: 12345,
            is_bot: false,
            first_name: "firstName".to_string(),
            last_name: Some("lastName".to_string()),
            username: Some("Username".to_string()),
            language_code: Some("languageCode".to_string()),
        };
        let actual = serde_json::from_str::<User>(&json).unwrap();
        assert_eq!(actual, expected)
    }
}
