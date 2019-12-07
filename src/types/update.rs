#![allow(clippy::large_enum_variant)]

use crate::types::{CallbackQuery, ChosenInlineResult, Message};

#[derive(Debug, Deserialize, PartialEq, Clone)]
pub struct Update {
    #[serde(rename = "update_id")]
    pub id: i32,
    #[serde(flatten)]
    pub kind: UpdateKind,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum UpdateKind {
    Message(Message),
    EditedMessage(Message),
    ChannelPost(Message),
    EditedChannelPost(Message),
    InlineQuery(()),
    // TODO
    ChosenInlineResult(ChosenInlineResult),
    CallbackQuery(CallbackQuery),
}

#[cfg(test)]
mod test {
    use crate::types::{
        Chat, ChatKind, ForwardKind, MediaKind, Message, MessageKind, Sender,
        Update, UpdateKind, User,
    };

    // TODO: more tests for deserialization
    #[test]
    fn message() {
        let json = r#"{
            "update_id":892252934,
            "message":{
                "message_id":6557,
                "from":{
                    "id":218485655,
                    "is_bot": false,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "language_code":"en"
                },
                "chat":{
                    "id":218485655,
                    "first_name":"Waffle",
                    "username":"WaffleLapkin",
                    "type":"private"
                },
               "date":1569518342,
               "text":"hello there"
            }
        }"#;

        let expected: Update = Update {
            id: 892_252_934,
            kind: UpdateKind::Message(Message {
                id: 6557,
                date: 1_569_518_342,
                chat: Chat {
                    id: 218_485_655,
                    kind: ChatKind::Private {
                        type_: (),
                        username: Some(String::from("WaffleLapkin")),
                        first_name: Some(String::from("Waffle")),
                        last_name: None,
                    },
                    photo: None,
                },
                kind: MessageKind::Common {
                    from: Sender::User(User {
                        id: 218_485_655,
                        is_bot: false,
                        first_name: String::from("Waffle"),
                        last_name: None,
                        username: Some(String::from("WaffleLapkin")),
                        language_code: Some(String::from("en")),
                    }),
                    forward_kind: ForwardKind::Origin {
                        reply_to_message: None,
                    },
                    edit_date: None,
                    media_kind: MediaKind::Text {
                        text: String::from("hello there"),
                        entities: vec![],
                    },
                    reply_markup: None,
                },
            }),
        };

        let actual = serde_json::from_str::<Update>(json).unwrap();
        assert_eq!(expected, actual);
    }
}
