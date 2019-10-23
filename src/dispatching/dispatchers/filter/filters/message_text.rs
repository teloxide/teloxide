use crate::dispatching::Filter;
use crate::types::Message;

/// Filter which compare message text with another text.
/// Returns true if the message text is equal to another text, otherwise false.
///
/// NOTE: filter compares only text message, does not compare caption of media!
///
/// If you want to compare caption use
/// [MessageCaptionFilter]
///
/// If you want to compare text and caption use
/// [MessageTextCaptionFilter]
///
/// [MessageCaptionFilter]: telebofr::dispatching::dispatchers::filter::filters::MessageCaptionFilter
/// [MessageTextCaptionFilter]: telebofr::dispatching::dispatchers::filter::filters::MessageTextCaptionFilter
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
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>
    {
        Self {
            text: text.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn texts_are_equal() {
        let filter = MessageTextFilter::new("text");
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
        let filter = MessageTextFilter::new("text");
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