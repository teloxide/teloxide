use teloxide::{
    dispatching2::dialogue::{serializer::Bincode, RedisStorage, Storage},
    macros::DialogueState,
    prelude2::*,
    RequestError,
};
use thiserror::Error;

type MyDialogue = Dialogue<State, RedisStorage<Bincode>>;
type StorageError = <RedisStorage<Bincode> as Storage<State>>::Error;

#[derive(Debug, Error)]
enum Error {
    #[error("error from Telegram: {0}")]
    TelegramError(#[from] RequestError),

    #[error("error from storage: {0}")]
    StorageError(#[from] StorageError),
}

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_start)]
    Start,

    #[handler(handle_got_number)]
    GotNumber(i32),
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::from_env().auto_send();
    // You can also choose serializer::JSON or serializer::CBOR
    // All serializers but JSON require enabling feature
    // "serializer-<name>", e. g. "serializer-cbor"
    // or "serializer-bincode"
    let storage = RedisStorage::open("redis://127.0.0.1:6379", Bincode).await.unwrap();

    let handler = Update::filter_message()
        .add_dialogue::<Message, RedisStorage<Bincode>, State>()
        .dispatch_by::<State>();

    DispatcherBuilder::new(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

async fn handle_start(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> anyhow::Result<()> {
    match msg.text().unwrap().parse() {
        Ok(number) => {
            dialogue.update(State::GotNumber(number)).await?;
            bot.send_message(
                msg.chat.id,
                format!("Remembered number {}. Now use /get or /reset", number),
            )
            .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Please, send me a number").await?;
        }
    }

    Ok(())
}

async fn handle_got_number(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    num: i32,
) -> anyhow::Result<()> {
    let ans = msg.text().unwrap();

    if ans.starts_with("/get") {
        bot.send_message(msg.chat.id, format!("Here is your number: {}", num)).await?;
    } else if ans.starts_with("/reset") {
        dialogue.reset().await?;
        bot.send_message(msg.chat.id, "Resetted number").await?;
    } else {
        bot.send_message(msg.chat.id, "Please, send /get or /reset").await?;
    }

    Ok(())
}
