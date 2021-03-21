#[macro_use]
extern crate derive_more;

mod states;
mod transitions;

use states::*;

use teloxide::{
    dispatching::dialogue::{serializer::Json, SqliteStorage, Storage},
    prelude::*,
    RequestError,
};
use thiserror::Error;

type StorageError = <SqliteStorage<Json> as Storage<Dialogue>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

type In = DialogueWithCx<AutoSend<Bot>, Message, Dialogue, StorageError>;

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        None => {
            cx.answer("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::with_storage(
            |DialogueWithCx { cx, dialogue }: In| async move {
                let dialogue = dialogue.expect("std::convert::Infallible");
                handle_message(cx, dialogue).await.expect("Something wrong with the bot!")
            },
            SqliteStorage::open("db.sqlite", Json).await.unwrap(),
        ))
        .dispatch()
        .await;
}
