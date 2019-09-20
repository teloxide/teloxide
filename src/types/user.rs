#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub is_bot: bool,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
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
