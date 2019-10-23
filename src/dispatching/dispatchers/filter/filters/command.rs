use crate::dispatching::Filter;
use crate::types::Message;
use std::ops::Add;

pub struct CommandFilter {
    command: String,
}

impl Filter<Message> for CommandFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => {
                match text.split_whitespace().next() {
                    Some(command) => self.command == command,
                    None => false
                }
            }
            None => false
        }
    }
}

impl CommandFilter {
    pub fn new<T>(command: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            command: '/'.to_string() + command.into().as_str()
        }
    }
    pub fn with_start_string<T>(command: T, start_string: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            command: start_string.into().add(command.into().as_str())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_are_equal() {
        let filter = CommandFilter::new("command".to_string());
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
          "text": "/command"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert!(filter.test(&message));
    }

    #[test]
    fn commands_are_not_equal() {
        let filter = CommandFilter::new("command".to_string());
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
          "text": "/command_not_equal"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(filter.test(&message), false);
    }

    #[test]
    fn command_have_args() {
        let filter = CommandFilter::new("command".to_string());
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
          "text": "/command arg1 arg2"
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert!(filter.test(&message));
    }

    #[test]
    fn message_have_only_whitespace() {
        let filter = CommandFilter::new("command".to_string());
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
          "text": " "
         }"#;
        let message = serde_json::from_str::<Message>(json).unwrap();
        assert_eq!(filter.test(&message), false);
    }
}
