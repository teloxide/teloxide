use crate::dispatching::Filter;
use crate::types::Message;

pub struct MessageCaptionFilter {
    text: String,
}

impl Filter<Message> for MessageCaptionFilter {
    fn test(&self, value: &Message) -> bool {
        match value.caption() {
            Some(caption) => self.text == caption,
            None => false
        }
    }
}

impl MessageCaptionFilter {
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
    fn captions_are_equal() {
        let filter = MessageCaptionFilter::new("caption".to_string());
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
          "photo": [
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA20AAybcBAABFgQ",
            "file_size": 18188,
            "width": 320,
            "height": 239
           }
          ],
          "caption": "caption"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert!(filter.test(&message));
    }

    #[test]
    fn captions_are_not_equal() {
        let filter = MessageCaptionFilter::new("text".to_string());
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
          "photo": [
           {
            "file_id": "AgADAgAD36sxG-PX0UvQSXIn9rccdw-ACA4ABAEAAwIAA20AAybcBAABFgQ",
            "file_size": 18188,
            "width": 320,
            "height": 239
           }
          ],
          "caption": "not equal caption"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(filter.test(&message), false);
    }
}