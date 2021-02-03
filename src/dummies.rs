use crate::types::{Message, Update, UpdateKind};

pub fn message_update(mes: Message) -> Update {
    Update::new(0, UpdateKind::Message(mes))
}

pub fn text_message<T: Into<String>>(text: T) -> Message {
    use crate::types::{
        ChatKind::Private, ForwardKind::Origin, MediaKind::Text, MessageKind::Common, *,
    };

    Message {
        id: 199785,
        date: 1568289890,
        chat: Chat {
            id: 250918540,
            kind: Private(ChatPrivate {
                type_: (),
                username: Some("aka_dude".into()),
                first_name: Some("Андрей".into()),
                last_name: Some("Власов".into()),
            }),
            photo: None,
        },
        via_bot: None,
        kind: Common(MessageCommon {
            from: Some(User {
                id: 250918540,
                is_bot: false,
                first_name: "Андрей".into(),
                last_name: Some("Власов".into()),
                username: Some("aka_dude".into()),
                language_code: Some("en".into()),
            }),
            forward_kind: Origin(ForwardOrigin { reply_to_message: None }),
            edit_date: None,
            media_kind: Text(MediaText { text: text.into(), entities: vec![] }),
            reply_markup: None,
        }),
    }
}
