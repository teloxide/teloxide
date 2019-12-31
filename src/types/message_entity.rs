use serde::{Deserialize, Serialize};

use crate::types::User;

/// This object represents one special entity in a text message. For example,
/// hashtags, usernames, URLs, etc.
///
/// [The official docs](https://core.telegram.org/bots/api#messageentity).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,

    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: usize,

    /// Length of the entity in UTF-16 code units.
    pub length: usize,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
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
