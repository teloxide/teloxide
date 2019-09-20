use crate::core::types::{Message, User};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub chat_instance: String,
    pub message: Option<Message>,
    pub inline_message_id: Option<String>,
    pub data: Option<String>,
    pub game_short_name: Option<String>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "id":"id",
            "from":{
                "id":12345,
                "is_bot":false,
                "first_name":"firstName"
            },
            "inline_message_id":"i_m_id",
            "chat_instance":"123456",
            "data":"some_data",
            "game_short_name":"game_name"
        }"#;
        let expected = CallbackQuery {
            id: "id".to_string(),
            from: User {
                id: 12345,
                is_bot: false,
                first_name: "firstName".to_string(),
                last_name: None,
                username: None,
                language_code: None
            },
            chat_instance: "123456".to_string(),
            message: None,
            inline_message_id: Some("i_m_id".to_string()),
            data: Some("some_data".to_string()),
            game_short_name: Some("game_name".to_string())
        };
        let actual = serde_json::from_str::<CallbackQuery>(json).unwrap();
        assert_eq!(actual, expected);
    }
}
