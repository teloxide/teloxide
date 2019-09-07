use crate::core::types::User;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
pub struct MessageEntity {
    #[serde(flatten)]
    pub kind: MessageEntityKind,
    pub offset: usize,
    pub length: usize,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone)]
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
