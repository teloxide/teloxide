use crate::dispatching::Filter;
use crate::types::Message;

/// Filter which compare message text or caption of media with another text.
/// Returns true if the message text or caption of media is equal to another text, otherwise false.
///
/// NOTE: filter compares text of message or if it is not exists, compares caption of the message!
///
/// If you want to compare only caption use
/// [MessageCaptionFilter]
///
/// If you want to compare only text use
/// [MessageTextFilter]
///
/// [MessageCaptionFilter]: telebofr::dispatching::dispatchers::filter::filters::MessageCaptionFilter
/// [MessageTextFilter]: telebofr::dispatching::dispatchers::filter::filters::MessageTextFilter
pub struct MessageTextCaptionFilter {
    text: String,
}

impl Filter<Message> for MessageTextCaptionFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => self.text == text,
            None => {
                match value.caption() {
                    Some(caption) => self.text == caption,
                    None => false
                }
            }
        }
    }
}

impl MessageTextCaptionFilter {
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
        let filter = MessageTextCaptionFilter::new("text");
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
        let filter = MessageTextCaptionFilter::new("text");
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

    #[test]
    fn captions_are_equal() {
        let filter = MessageTextCaptionFilter::new("caption".to_string());
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
        let filter = MessageTextCaptionFilter::new("text".to_string());
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