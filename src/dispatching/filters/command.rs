use crate::{dispatching::Filter, types::Message};

/// Filter which find command in message text
///
/// *NB:* filter compare only text of message, not caption of media message
///
/// Examples:
/// ```
/// use teloxide::dispatching::filters::CommandFilter;
/// CommandFilter::new("start"); // return true if text message starts with "/start"
/// CommandFilter::with_prefix("!", "ban"); // return true if text message starts with "!ban"
/// ```
pub struct CommandFilter {
    command: String,
}

impl Filter<Message> for CommandFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => text.starts_with(&self.command),
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
    pub fn with_prefix<T, U>(prefix: U, command: T) -> Self
    where
        T: Into<String>,
        U: Into<String>,
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
        let filter = CommandFilter::new("command");
        let message = create_message_with_text("/command".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn commands_are_not_equal() {
        let filter = CommandFilter::new("command");
        let message =
            create_message_with_text("/not_equal_command".to_string());
        assert_eq!(filter.test(&message), false);
    }

    #[test]
    fn command_have_args() {
        let filter = CommandFilter::new("command");
        let message =
            create_message_with_text("/command arg1 arg2".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn message_have_only_whitespace() {
        let filter = CommandFilter::new("command");
        let message = create_message_with_text(" ".to_string());
        assert_eq!(filter.test(&message), false);
    }

    #[test]
    fn another_prefix() {
        let filter = CommandFilter::with_prefix("command", "!");
        let message = create_message_with_text("!command".to_string());
        assert!(filter.test(&message));
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
