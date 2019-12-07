use crate::{dispatching::Filter, types::Message};

/// Filter which compare message text or caption of media with another text.
/// Returns true if the message text or caption of media is equal to another
/// text, otherwise false.
///
/// NOTE: filter compares text of message or if it is not exists, compares
/// caption of the message!
///
/// If you want to compare only caption use
/// [MessageCaptionFilter]
///
/// If you want to compare only text use
/// [MessageTextFilter]
///
/// [MessageCaptionFilter]: crate::dispatching::filters::MessageCaptionFilter
/// [MessageTextFilter]: crate::dispatching::filters::MessageTextFilter
pub struct MessageTextCaptionFilter {
    text: String,
}

impl Filter<Message> for MessageTextCaptionFilter {
    fn test(&self, value: &Message) -> bool {
        match value.text() {
            Some(text) => self.text == text,
            None => match value.caption() {
                Some(caption) => self.text == caption,
                None => false,
            },
        }
    }
}

impl MessageTextCaptionFilter {
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
        let filter = MessageTextCaptionFilter::new("text");
        let message = create_message_with_text("text".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn texts_are_not_equal() {
        let filter = MessageTextCaptionFilter::new("text");
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

    #[test]
    fn captions_are_equal() {
        let filter = MessageTextCaptionFilter::new("caption".to_string());
        let message = create_message_with_caption("caption".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn captions_are_not_equal() {
        let filter = MessageTextCaptionFilter::new("caption".to_string());
        let message =
            create_message_with_caption("not equal caption".to_string());
        assert_eq!(filter.test(&message), false);
    }

    fn create_message_with_caption(caption: String) -> Message {
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
                media_kind: MediaKind::Photo {
                    photo: vec![],
                    caption: Some(caption),
                    caption_entities: vec![],
                    media_group_id: None,
                },
                reply_markup: None,
            },
        }
    }
}
