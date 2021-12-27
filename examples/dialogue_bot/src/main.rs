// This is a bot that asks you three questions, e.g. a simple test.
//
// # Example
// ```
//  - Hey
//  - Let's start! What's your full name?
//  - Gandalf the Grey
//  - How old are you?
//  - 223
//  - What's your location?
//  - Middle-earth
//  - Full name: Gandalf the Grey
//    Age: 223
//    Location: Middle-earth
// ```
use teloxide::{
    dispatching2::dialogue::{serializer::Json, SqliteStorage},
    prelude::*,
};

// FIXME: naming
type MyBot = AutoSend<Bot>;
type Store = SqliteStorage<Json>;
type BotDialogue = Dialogue<State, Store>;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum State {
    Start,
    ReceiveFullName,
    ReceiveAge { full_name: String },
    ReceiveLocation { full_name: String, age: u8 },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();
    let storage = SqliteStorage::open("db.sqlite", Json).await.unwrap();

    Dispatcher::new(bot)
        .dependencies(dptree::deps![storage])
        .messages_handler(|h| {
            h.add_dialogue::<Message, Store, State>().branch(dptree::endpoint(handle_message))
        })
        .dispatch()
        .await;
}

async fn handle_message(bot: MyBot, mes: Message, dialogue: BotDialogue) -> anyhow::Result<()> {
    match mes.text().map(ToOwned::to_owned) {
        None => {
            bot.send_message(mes.chat_id(), "Send me a text message.").await?;
            Ok(())
        }
        Some(_) => match dialogue.current_state_or_default().await? {
            State::Start => handle_start(bot, mes, dialogue).await,
            State::ReceiveFullName => handle_receive_full_name(bot, mes, dialogue).await,
            State::ReceiveAge { full_name } => {
                handle_receive_age(bot, mes, dialogue, full_name).await
            }
            State::ReceiveLocation { full_name, age } => {
                handle_receive_location(bot, mes, dialogue, full_name, age).await
            }
        },
    }
}

async fn handle_start(bot: MyBot, mes: Message, dialogue: BotDialogue) -> anyhow::Result<()> {
    bot.send_message(mes.chat_id(), "Let's start! What's your full name?").await?;
    dialogue.next(State::ReceiveFullName).await?;
    Ok(())
}

async fn handle_receive_full_name(
    bot: MyBot,
    mes: Message,
    dialogue: BotDialogue,
) -> anyhow::Result<()> {
    bot.send_message(mes.chat_id(), "How old are you?").await?;
    dialogue.next(State::ReceiveAge { full_name: mes.text().unwrap().into() }).await?;
    Ok(())
}

async fn handle_receive_age(
    bot: MyBot,
    mes: Message,
    dialogue: BotDialogue,
    full_name: String,
) -> anyhow::Result<()> {
    match mes.text().unwrap().parse::<u8>() {
        Ok(age) => {
            bot.send_message(mes.chat_id(), "What's your location?").await?;
            dialogue.next(State::ReceiveLocation { full_name, age }).await?;
        }
        _ => {
            bot.send_message(mes.chat_id(), "Send me a number.").await?;
        }
    }
    Ok(())
}

async fn handle_receive_location(
    bot: MyBot,
    mes: Message,
    dialogue: BotDialogue,
    full_name: String,
    age: u8,
) -> anyhow::Result<()> {
    let location = mes.text().unwrap();
    let message = format!("Full name: {}\nAge: {}\nLocation: {}", full_name, age, location);
    bot.send_message(mes.chat_id(), message).await?;
    dialogue.exit().await?;
    Ok(())
}
