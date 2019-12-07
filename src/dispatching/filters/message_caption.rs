use crate::{dispatching::Filter, types::Message};

/// Filter which compare caption of media with another text.
/// Returns true if the caption of media is equal to another text, otherwise
/// false.
///
/// NOTE: filter compares only caption of media, does not compare text of
/// message!
///
/// If you want to compare text of message use
/// [MessageTextFilter]
///
/// If you want to compare text and caption use
/// [MessageTextCaptionFilter]
///
/// [MessageTextFilter]: crate::dispatching::filters::MessageTextFilter
/// [MessageTextCaptionFilter]:
/// crate::dispatching::filters::MessageTextCaptionFilter
pub struct MessageCaptionFilter {
    text: String,
}

impl Filter<Message> for MessageCaptionFilter {
    fn test(&self, value: &Message) -> bool {
        match value.caption() {
            Some(caption) => self.text == caption,
            None => false,
        }
    }
}

impl MessageCaptionFilter {
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
    fn captions_are_equal() {
        let filter = MessageCaptionFilter::new("caption".to_string());
        let message = create_message_with_caption("caption".to_string());
        assert!(filter.test(&message));
    }

    #[test]
    fn captions_are_not_equal() {
        let filter = MessageCaptionFilter::new("caption".to_string());
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
