use teloxide::{
    dispatching2::dialogue::{serializer::Bincode, RedisStorage, Storage},
    prelude2::*,
    RequestError,
};
use thiserror::Error;

type Store = RedisStorage<Bincode>;
// FIXME: naming
type MyDialogue = Dialogue<BotDialogue, Store>;
type StorageError = <RedisStorage<Bincode> as Storage<BotDialogue>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),
    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum BotDialogue {
    Start,
    HaveNumber(i32),
}

impl Default for BotDialogue {
    fn default() -> Self {
        Self::Start
    }
}

async fn handle_message(
    bot: AutoSend<Bot>,
    mes: Message,
    dialogue: MyDialogue,
) -> Result<(), Error> {
    match mes.text() {
        None => {
            bot.send_message(mes.chat.id, "Send me a text message.").await?;
        }
        Some(ans) => {
            let state = dialogue.current_state_or_default().await?;
            match state {
                BotDialogue::Start => {
                    if let Ok(number) = ans.parse() {
                        dialogue.next(BotDialogue::HaveNumber(number)).await?;
                        bot.send_message(
                            mes.chat.id,
                            format!("Remembered number {}. Now use /get or /reset", number),
                        )
                        .await?;
                    } else {
                        bot.send_message(mes.chat.id, "Please, send me a number").await?;
                    }
                }
                BotDialogue::HaveNumber(num) => {
                    if ans.starts_with("/get") {
                        bot.send_message(mes.chat.id, format!("Here is your number: {}", num))
                            .await?;
                    } else if ans.starts_with("/reset") {
                        dialogue.reset().await?;
                        bot.send_message(mes.chat.id, "Resetted number").await?;
                    } else {
                        bot.send_message(mes.chat.id, "Please, send /get or /reset").await?;
                    }
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    // You can also choose serializer::JSON or serializer::CBOR
    // All serializers but JSON require enabling feature
    // "serializer-<name>", e. g. "serializer-cbor"
    // or "serializer-bincode"
    let storage = RedisStorage::open("redis://127.0.0.1:6379", Bincode).await.unwrap();

    let handler = dptree::entry()
        .add_dialogue::<Message, Store, BotDialogue>()
        .branch(dptree::endpoint(handle_message));

    DispatcherBuilder::new(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .dispatch()
        .await;
}
