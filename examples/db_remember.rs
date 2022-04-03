// Set the `DB_REMEMBER_REDIS` environmental variable if you want to use Redis.
// Otherwise, the default is Sqlite.

use teloxide::{
    dispatching::dialogue::{
        serializer::{Bincode, Json},
        ErasedStorage, RedisStorage, SqliteStorage, Storage,
    },
    prelude::*,
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, ErasedStorage<State>>;
type MyStorage = std::sync::Arc<ErasedStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start,
    GotNumber(i32),
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[derive(Clone, BotCommands)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "get your number.")]
    Get,
    #[command(description = "reset your number.")]
    Reset,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting db_remember_bot...");

    let bot = Bot::from_env().auto_send();

    let storage: MyStorage = if std::env::var("DB_REMEMBER_REDIS").is_ok() {
        RedisStorage::open("redis://127.0.0.1:6379", Bincode).await.unwrap().erase()
    } else {
        SqliteStorage::open("db.sqlite", Json).await.unwrap().erase()
    };

    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(teloxide::handler![State::Start].endpoint(start))
        .branch(
            teloxide::handler![State::GotNumber(n)]
                .branch(dptree::entry().filter_command::<Command>().endpoint(got_number))
                .branch(dptree::endpoint(invalid_command)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}

async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    match msg.text().map(|text| text.parse::<i32>()) {
        Some(Ok(n)) => {
            dialogue.update(State::GotNumber(n)).await?;
            bot.send_message(
                msg.chat.id,
                format!("Remembered number {}. Now use /get or /reset.", n),
            )
            .await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Please, send me a number.").await?;
        }
    }

    Ok(())
}

async fn got_number(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    num: i32,
    cmd: Command,
) -> HandlerResult {
    match cmd {
        Command::Get => {
            bot.send_message(msg.chat.id, format!("Here is your number: {}.", num)).await?;
        }
        Command::Reset => {
            dialogue.reset().await?;
            bot.send_message(msg.chat.id, "Number resetted.").await?;
        }
    }
    Ok(())
}

async fn invalid_command(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please, send /get or /reset.").await?;
    Ok(())
}
