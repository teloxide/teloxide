use serde::{Deserialize, Serialize};

use crate::types::{User, Message};

/// This object represents one special entity in a text message. For example,
/// hashtags, usernames, URLs, etc.
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
    Pre,
    TextLink { url: String },
    TextMention { user: User },
    Underline,
    Strikethrough,
}

impl MessageEntity {
    fn text_from(&self, message: Message) -> Option<String> {
        let text= message.text();
        match text {
            Some(text) => {
                let left = self.offset;
                let right = self.offset+self.length;
                Some(String::from(text)[left..right].to_string())
            }
            None => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Chat, ChatKind, MessageKind, Sender, ForwardKind, MediaKind};

    #[test]
    fn recursive_kind() {
        use serde_json::from_str;

        assert_eq!(
            MessageEntity {
                kind: MessageEntityKind::TextLink {
                    url: "ya.ru".into()
                },
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
    fn text_from() {
        let message = message();
        let expected = Some("yes".to_string());
        let entity = message.entities().unwrap()[0].clone();
        let actual = entity.text_from(message);
        assert_eq!(actual, expected);
    }

    fn message() -> Message {
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
                    text: "no yes no".to_string(),
                    entities: vec![MessageEntity {
                        kind: MessageEntityKind::Mention,
                        offset: 3,
                        length: 3
                    }],
                },
                reply_markup: None,
            },
        }
    }
}
