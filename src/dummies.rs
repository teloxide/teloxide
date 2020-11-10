use crate::{
    prelude::UpdateWithCx,
    types::{
        Chat, ChatKind::Private, ChatPrivate, ForwardKind::Origin, ForwardOrigin, MediaKind::Text,
        MediaText, Message, MessageCommon, MessageKind::Common, User,
    },
    Bot,
};
use reqwest::Client;

pub fn text_message<T: Into<String>>(text: T) -> Message {
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

pub fn bot() -> Bot {
    Bot::builder().token("token").client(Client::new()).build()
}

pub fn update_with_cx<T>(update: T) -> UpdateWithCx<T> {
    UpdateWithCx { bot: bot(), update }
}
