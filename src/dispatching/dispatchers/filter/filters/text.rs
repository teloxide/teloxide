use crate::dispatching::Filter;
use crate::types::Message;

pub struct MessageTextFilter {
    text: String,
}

impl Filter<Message> for MessageTextFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => self.text == text,
            None => false
        }
    }
}

impl MessageTextFilter {
    pub fn new(text: String) -> Self {
        Self {
            text
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_are_equal() {
        let filter = MessageTextFilter::new("text".to_string());
        let json = r#"{
          "message_id": 199785,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568289890,
          "text": "text"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert!(filter.test(&message));
    }

    #[test]
    fn texts_are_not_equal() {
        let filter = MessageTextFilter::new("text".to_string());
        let json = r#"{
          "message_id": 199785,
          "from": {
           "id": 250918540,
           "is_bot": false,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "language_code": "en"
          },
          "chat": {
           "id": 250918540,
           "first_name": "Андрей",
           "last_name": "Власов",
           "username": "aka_dude",
           "type": "private"
          },
          "date": 1568289890,
          "text": "not equal text"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(filter.test(&message), false);
    }
}