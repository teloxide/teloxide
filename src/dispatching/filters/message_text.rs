use crate::{dispatching::Filter, types::Message};

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
/// [MessageCaptionFilter]: crate::dispatching::filters::MessageCaptionFilter
/// [MessageTextCaptionFilter]:
/// crate::dispatching::filters::MessageTextCaptionFilter
pub struct MessageTextFilter {
    text: String,
}

impl Filter<Message> for MessageTextFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => self.text == text,
            None => false,
        }
    }
}

impl MessageTextFilter {
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { text: text.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Chat, ChatKind, ForwardKind, MediaKind, MessageKind, Sender, User,
    };

    #[test]
    fn texts_are_equal() {
        let filter = MessageTextFilter::new("text");
        let message = create_message_with_text("text".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn texts_are_not_equal() {
        let filter = MessageTextFilter::new("text");
        let message = create_message_with_text("not equal text".to_string());
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
