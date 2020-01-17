use regex::Regex;
use crate::dispatching::Filter;
use crate::types::Message;

// TODO: docs
pub struct RegexFilter {
    regexp: Regex
}

impl Filter<Message> for RegexFilter {
    fn test(&self, value: &Message) -> bool {
        self.regexp.is_match(value.text()?)
    }
}

impl RegexFilter {
    pub fn new(regexp: Regex) -> Self {
        Self {
            regexp
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
    fn match_true() {
        let filter = RegexFilter::new(Regex::new(r"\w+").unwrap());
        let message = create_message_with_text("text".to_string());
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