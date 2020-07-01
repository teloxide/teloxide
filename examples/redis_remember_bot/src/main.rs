#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate derive_more;

mod states;
mod transitions;

use states::*;
use transitions::*;

use std::sync::Arc;
use teloxide::{
    dispatching::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude::*,
};
use thiserror::Error;

type StorageError = <RedisStorage<Bincode> as Storage<Dialogue>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

type In = TransitionIn<Dialogue, StorageError>;

async fn handle_message(input: In) -> Out {
    let (cx, dialogue) = input.unpack();

    let text = match cx.update.text_owned() {
        Some(text) => text,
        None => {
            cx.answer_str("Please, send me a text message").await?;
            return next(StartState);
        }
    };

    dispatch(cx, dialogue, &text).await
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            |cx| async move {
                handle_message(cx)
                    .await
                    .expect("Something is wrong with the bot!")
            },
            Arc::new(
                // You can also choose serializer::JSON or serializer::CBOR
                // All serializers but JSON require enabling feature
                // "serializer-<name>", e. g. "serializer-cbor"
                // or "serializer-bincode"
                RedisStorage::open("redis://127.0.0.1:6379", Bincode)
                    .await
                    .unwrap(),
            ),
        ))
        .dispatch()
        .await;
}
