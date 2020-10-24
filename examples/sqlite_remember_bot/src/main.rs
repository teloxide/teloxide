#[macro_use]
extern crate derive_more;

mod states;
mod transitions;

use states::*;

use teloxide::{
    dispatching::dialogue::{serializer::JSON, SqliteStorage, Storage},
    prelude::*,
};
use thiserror::Error;

type StorageError = <SqliteStorage<JSON> as Storage<Dialogue>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

type In = DialogueWithCx<Message, Dialogue, StorageError>;

async fn handle_message(cx: UpdateWithCx<Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    match cx.update.text_owned() {
        None => {
            cx.answer_str("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env();
    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            |DialogueWithCx { cx, dialogue }: In| async move {
                let dialogue = dialogue.expect("std::convert::Infallible");
                handle_message(cx, dialogue).await.expect("Something wrong with the bot!")
            },
            SqliteStorage::open("sqlite.db", JSON).await.unwrap(),
        ))
        .dispatch()
        .await;
}
