use crate::{
    dispatching::{updates, DispatcherBuilder},
    types::{Message, Update, UpdateKind},
};
use std::{
    convert::Infallible,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[tokio::test]
async fn test() {
    let handled = Arc::new(AtomicBool::new(false));
    let handled2 = handled.clone();

    let dispatcher = DispatcherBuilder::<Infallible, _>::new()
        .handle(updates::message().common().by(move |message: Message| {
            assert_eq!(message.text().unwrap(), "text");
            handled2.store(true, Ordering::SeqCst);
        }))
        .handle(updates::callback_query().by(move || unreachable!()))
        .error_handler(|_| async { unreachable!() })
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text")));

    dispatcher.dispatch_one(message).await;

    assert!(handled.load(Ordering::SeqCst));
}

#[tokio::test]
async fn or_else() {
    let in_or_else = Arc::new(AtomicBool::new(false));

    let dispatcher = DispatcherBuilder::<Infallible, _>::new()
        .handle(
            updates::message()
                .common()
                .with_text(|text: &str| text == "text")
                .or_else({
                    let in_or_else = in_or_else.clone();
                    move || {
                        in_or_else.store(true, Ordering::SeqCst);
                    }
                })
                .by(|| unreachable!()),
        )
        .error_handler(|_| async { unreachable!() })
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("not_text")));

    dispatcher.dispatch_one(message).await;

    assert!(in_or_else.load(Ordering::SeqCst));
}

#[tokio::test]
async fn or() {
    let handled = Arc::new(AtomicBool::new(false));

    let dispatcher = DispatcherBuilder::<Infallible, _>::new()
        .handle(
            updates::message()
                .common()
                .with_text(|text: &str| text == "text")
                .or_with_text(|text: &str| text == "text2")
                .by({
                    let handled = handled.clone();
                    move || handled.store(true, Ordering::SeqCst)
                }),
        )
        .error_handler(|_| async { unreachable!() })
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text2")));

    dispatcher.dispatch_one(message).await;

    assert!(handled.load(Ordering::SeqCst));
}

fn text_message<T: Into<String>>(text: T) -> Message {
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
