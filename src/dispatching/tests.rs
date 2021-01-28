use crate::{
    dispatching::{updates, DispatcherBuilder, UpdateWithCx},
    dummies::text_message,
    types::{Message, Update, UpdateKind},
    Bot,
};
use std::{
    convert::Infallible,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use crate::dispatching::tel;
use crate::dispatching::dialogue::{DialogueDispatcherBuilder, InMemStorage, DialogueWithCx, DialogueStage, DialogueHandlerBuilderExt};

#[tokio::test]
async fn test() {
    let handled = Arc::new(AtomicBool::new(false));
    let handled2 = handled.clone();

    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .handle(updates::message().by(move |message: Message| {
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

    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .handle(
            updates::message()
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

    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .handle(
            updates::message()
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

#[tokio::test]
async fn async_guards() {
    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .handle(
            updates::message()
                .with_chat_id(|id: &i64| {
                    let id = id.clone();
                    async move { id == 10 }
                })
                .by(|mes: Message| assert_eq!(mes.chat.id, 10)),
        )
        .error_handler(|_| async {})
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text2")));

    dispatcher.dispatch_one(message).await;
}

#[tokio::test]
async fn update_with_cx() {
    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .handle(
            updates::message()
                .by(|cx: UpdateWithCx<Message>| assert_eq!(cx.update.text().unwrap(), "text2")),
        )
        .error_handler(|_| async {})
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text2")));

    dispatcher.dispatch_one(message).await;
}

#[tokio::test]
async fn global_data() {
    struct SomeData(u8);

    let dispatcher = DispatcherBuilder::<Infallible, _>::new(dummy_bot(), "bot_name")
        .data(SomeData(1))
        .handle(
            updates::message()
                .by(|_: Message, data: tel::Data<SomeData>| assert_eq!((data.0).0, 1)),
        )
        .error_handler(|_| async { unreachable!() })
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text2")));

    dispatcher.dispatch_one(message).await;
}

#[tokio::test]
async fn with_dialogue() {
    #[derive(Debug, PartialEq, Clone)]
    enum Dialogue {
        Start,
        HaveData(u8),
    }
    impl Default for Dialogue {
        fn default() -> Self {
            Self::Start
        }
    }

    let dispatcher = DialogueDispatcherBuilder::new(
        dummy_bot(),
        "bot_name",
        InMemStorage::new()
    )
        .handle(
            updates::message()
                .with_dialogue(|d: &Dialogue| matches!(d, Dialogue::Start))
                .by(|DialogueWithCx { dialogue, .. }: DialogueWithCx<Message, Dialogue, Infallible>| {
                    assert_eq!(dialogue.data.as_ref().unwrap(), &Dialogue::Start);
                    async move {
                        dialogue.next(|_| DialogueStage::Next(Dialogue::HaveData(10))).await.unwrap();
                    }
                }),
        )
        .handle(
            updates::message()
                .with_dialogue(|d: &Dialogue| matches!(d, Dialogue::HaveData(_)))
                .by(|DialogueWithCx { dialogue, .. }: DialogueWithCx<Message, Dialogue, Infallible>| {
                    assert_eq!(dialogue.data.as_ref().unwrap(), &Dialogue::HaveData(10));
                    async move {
                        dialogue.next(|_| DialogueStage::Exit).await.unwrap();
                    }
                }),
        )
        .error_handler(|_| async { unreachable!() })
        .build();

    let message = Update::new(0, UpdateKind::Message(text_message("text2")));

    dispatcher.dispatch_one(message.clone()).await;
    dispatcher.dispatch_one(message.clone()).await;
    dispatcher.dispatch_one(message.clone()).await;
    dispatcher.dispatch_one(message.clone()).await;
}

fn dummy_bot() -> Bot {
    Bot::builder().token("").build()
}
