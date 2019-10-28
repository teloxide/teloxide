use crate::{dispatching::Filter, types::Message};

pub struct CommandFilter {
    command: String,
}

impl Filter<Message> for CommandFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => match text.split_whitespace().next() {
                Some(command) => self.command == command,
                None => false,
            },
            None => false,
        }
    }
}

impl CommandFilter {
    pub fn new<T>(command: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            command: '/'.to_string() + &command.into(),
        }
    }
    pub fn with_prefix<T>(command: T, prefix: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            command: prefix.into() + &command.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Chat, ChatKind, ForwardKind, MediaKind, MessageKind, Sender, User,
    };

    #[test]
    fn commands_are_equal() {
        let filter = CommandFilter::new("command".to_string());
        let message = create_message_with_text("/command".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn commands_are_not_equal() {
        let filter = CommandFilter::new("command".to_string());
        let message =
            create_message_with_text("/not_equal_command".to_string());
        assert_eq!(filter.test(&message), false);
    }

    #[test]
    fn command_have_args() {
        let filter = CommandFilter::new("command".to_string());
        let message =
            create_message_with_text("/command arg1 arg2".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn message_have_only_whitespace() {
        let filter = CommandFilter::new("command".to_string());
        let message = create_message_with_text(" ".to_string());
        assert_eq!(filter.test(&message), false);
    }

    fn create_message_with_text(text: String) -> Message {
        Message {
            id: 0,
            date: 0,
            chat: Chat {
                id: 0,
                kind: ChatKind::Private {
                    type_: (),
                    username: None,
                    first_name: None,
                    last_name: None,
                },
                photo: None,
            },
            kind: MessageKind::Common {
                from: Sender::User(User {
                    id: 0,
                    is_bot: false,
                    first_name: "".to_string(),
                    last_name: None,
                    username: None,
                    language_code: None,
                }),
                forward_kind: ForwardKind::Origin {
                    reply_to_message: None,
                },
                edit_date: None,
                media_kind: MediaKind::Text {
                    text,
                    entities: vec![],
                },
                reply_markup: None,
            },
        }
    }
}
