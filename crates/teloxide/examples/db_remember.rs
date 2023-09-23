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

#[derive(Clone, Default, serde::Serialize, serde::Deserialize)]
pub enum State {
    #[default]
    Start,
    GotNumber(i32),
}

/// These commands are supported:
#[derive(Clone, BotCommands)]
#[command(rename_rule = "lowercase")]
pub enum Command {
    /// Get your number.
    Get,
    /// Reset your number.
    Reset,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting DB remember bot...");

    let bot = Bot::from_env();

    let storage: MyStorage = if std::env::var("DB_REMEMBER_REDIS").is_ok() {
        RedisStorage::open("redis://127.0.0.1:6379", Bincode).await.unwrap().erase()
    } else {
        SqliteStorage::open("db.sqlite", Json).await.unwrap().erase()
    };

    let handler = Update::filter_message()
        .enter_dialogue::<Message, ErasedStorage<State>, State>()
        .branch(dptree::case![State::Start].endpoint(start))
        .branch(
            dptree::case![State::GotNumber(n)]
                .branch(dptree::entry().filter_command::<Command>().endpoint(got_number))
                .branch(dptree::endpoint(invalid_command)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![storage])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    match msg.text().map(|text| text.parse::<i32>()) {
        Some(Ok(n)) => {
            dialogue.update(State::GotNumber(n)).await?;
            bot.send_message(
                msg.chat.id,
                format!("Remembered number {n}. Now use /get or /reset."),
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
    bot: Bot,
    dialogue: MyDialogue,
    num: i32, // Available from `State::GotNumber`.
    msg: Message,
    cmd: Command,
) -> HandlerResult {
    match cmd {
        Command::Get => {
            bot.send_message(msg.chat.id, format!("Here is your number: {num}.")).await?;
        }
        Command::Reset => {
            dialogue.reset().await?;
            bot.send_message(msg.chat.id, "Number reset.").await?;
        }
    }
    Ok(())
}

async fn invalid_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Please, send /get or /reset.").await?;
    Ok(())
}
