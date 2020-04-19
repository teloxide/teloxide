use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::sync::Arc;
use teloxide::{
    dispatching::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude::*,
};
use thiserror::Error;

#[derive(SmartDefault, Serialize, Deserialize)]
enum Dialogue {
    #[default]
    Start,
    HaveNumber(i32),
}

type StorageError = <RedisStorage<Bincode> as Storage<Dialogue>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

type Cx<State> = DialogueDispatcherHandlerCx<Message, State, StorageError>;

type Res = Result<DialogueStage<Dialogue>, Error>;

async fn handle_message(cx: Cx<Dialogue>) -> Res {
    let DialogueDispatcherHandlerCx { bot, update, dialogue } = cx;
    let text = match update.text() {
        Some(text) => text,
        None => {
            bot.send_message(
                update.chat_id(),
                "Please, send me a text message",
            )
            .send()
            .await?;
            return next(Dialogue::Start);
        }
    };

    match dialogue? {
        Dialogue::Start => {
            if let Ok(number) = text.parse() {
                bot.send_message(
                    update.chat_id(),
                    format!(
                        "Remembered number {}. Now use /get or /reset",
                        number
                    ),
                )
                .send()
                .await?;
                next(Dialogue::HaveNumber(number))
            } else {
                bot.send_message(update.chat_id(), "Please, send me a number")
                    .send()
                    .await?;
                next(Dialogue::Start)
            }
        }
        Dialogue::HaveNumber(num) => {
            if text.starts_with("/get") {
                bot.send_message(
                    update.chat_id(),
                    format!("Here is your number: {}", num),
                )
                .send()
                .await?;
                next(Dialogue::HaveNumber(num))
            } else if text.starts_with("/reset") {
                bot.send_message(update.chat_id(), format!("Resetted number"))
                    .send()
                    .await?;
                next(Dialogue::Start)
            } else {
                bot.send_message(
                    update.chat_id(),
                    "Please, send /get or /reset",
                )
                .send()
                .await?;
                next(Dialogue::HaveNumber(num))
            }
        }
    }
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
