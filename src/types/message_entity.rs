use serde::{Deserialize, Serialize};

use crate::types::{Message, User};

/// This object represents one special entity in a text message.
///
/// For example, hashtags, usernames, URLs, etc.
///
/// [The official docs](https://core.telegram.org/bots/api#messageentity).
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,

    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: usize,

    /// Length of the entity in UTF-16 code units.
    pub length: usize,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    Cashtag,
    BotCommand,
    Url,
    Email,
    PhoneNumber,
    Bold,
    Italic,
    Code,
    Pre { language: Option<String> },
    TextLink { url: String },
    TextMention { user: User },
    Underline,
    Strikethrough,
}

impl MessageEntity {
    pub fn text_from(&self, message: &Message) -> Option<String> {
        let text = message.text();
        Some(String::from(&text?[self.offset..self.offset + self.length]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{
        Chat, ChatKind, ChatPrivate, ForwardKind, ForwardOrigin, MediaKind,
        MediaText, MessageCommon, MessageKind,
    };

    #[test]
    fn recursive_kind() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::TextLink { url: "ya.ru".into() },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(
                r#"{"type":"text_link","url":"ya.ru","offset":1,"length":2}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn pre() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::Pre {
                    language: Some("rust".to_string()),
                },
                offset: 1,
                length: 2,
            },
            from_str::<MessageEntity>(
                r#"{"type":"pre","url":"ya.ru","offset":1,"length":2,"language":"rust"}"#
            )
            .unwrap()
        );
    }

    #[test]
    fn text_from() {
        let message = message();
        let expected = Some("yes".to_string());
        let entity = message.entities().unwrap()[0].clone();
        let actual = entity.text_from(&message);
        assert_eq!(actual, expected);
    }

    fn message() -> Message {
        Message {
            id: 0,
            date: 0,
            chat: Chat {
                id: 0,
                kind: ChatKind::Private(ChatPrivate {
                    type_: (),
                    username: None,
                    first_name: None,
                    last_name: None,
                }),
                photo: None,
            },
            kind: MessageKind::Common(MessageCommon {
                from: Some(User {
                    id: 0,
                    is_bot: false,
                    first_name: "".to_string(),
                    last_name: None,
                    username: None,
                    language_code: None,
                }),
                forward_kind: ForwardKind::Origin(ForwardOrigin {
                    reply_to_message: None,
                }),
                edit_date: None,
                media_kind: MediaKind::Text(MediaText {
                    text: "no yes no".to_string(),
                    entities: vec![MessageEntity {
                        kind: MessageEntityKind::Mention,
                        offset: 3,
                        length: 3,
                    }],
                }),
                reply_markup: None,
            }),
        }
    }
}
