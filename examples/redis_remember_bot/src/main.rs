#[macro_use]
extern crate smart_default;
#[macro_use]
extern crate derive_more;

mod states;
mod transitions;

use states::*;

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

type In = DialogueWithCx<Message, Dialogue, StorageError>;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let bot = Bot::from_env();
    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            |DialogueWithCx { cx, dialogue }: In| async move {
                // No panic because of std::convert::Infallible.
                let dialogue = dialogue.unwrap();
                handle_message(cx, dialogue)
                    .await
                    .expect("Something wrong with the bot!")
            },
            // You can also choose serializer::JSON or serializer::CBOR
            // All serializers but JSON require enabling feature
            // "serializer-<name>", e. g. "serializer-cbor"
            // or "serializer-bincode"
            RedisStorage::open("redis://127.0.0.1:6379", Bincode)
                .await
                .unwrap(),
        ))
        .dispatch()
        .await;
}

async fn handle_message(
    cx: UpdateWithCx<Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text_owned() {
        None => {
            cx.answer_str("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}
