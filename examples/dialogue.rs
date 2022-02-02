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
    macros::DialogueState,
    prelude2::*,
};

type BotDialogue = Dialogue<State, SqliteStorage<Json>>;

#[derive(DialogueState, Clone, serde::Serialize, serde::Deserialize)]
#[handler_out(anyhow::Result<()>)]
pub enum State {
    #[handler(handle_start)]
    Start,

    #[handler(handle_receive_full_name)]
    ReceiveFullName,

    #[handler(handle_receive_age)]
    ReceiveAge(String),

    #[handler(handle_receive_location)]
    ReceiveLocation(ReceiveLocation),
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct ReceiveLocation {
    full_name: String,
    age: u8,
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

    DispatcherBuilder::new(
        bot,
        dptree::entry()
            .add_dialogue::<Message, SqliteStorage<Json>, State>()
            .dispatch_by::<State>(),
    )
    .dependencies(dptree::deps![storage])
    .build()
    .dispatch()
    .await;
}

async fn handle_start(
    bot: AutoSend<Bot>,
    mes: Message,
    dialogue: BotDialogue,
) -> anyhow::Result<()> {
    bot.send_message(mes.chat_id(), "Let's start! What's your full name?").await?;
    dialogue.next(State::ReceiveFullName).await?;
    Ok(())
}

async fn handle_receive_full_name(
    bot: AutoSend<Bot>,
    mes: Message,
    dialogue: BotDialogue,
) -> anyhow::Result<()> {
    bot.send_message(mes.chat_id(), "How old are you?").await?;
    dialogue.next(State::ReceiveAge(mes.text().unwrap().into())).await?;
    Ok(())
}

async fn handle_receive_age(
    bot: AutoSend<Bot>,
    mes: Message,
    dialogue: BotDialogue,
    full_name: String,
) -> anyhow::Result<()> {
    match mes.text().unwrap().parse::<u8>() {
        Ok(age) => {
            bot.send_message(mes.chat_id(), "What's your location?").await?;
            dialogue.next(State::ReceiveLocation(ReceiveLocation { full_name, age })).await?;
        }
        _ => {
            bot.send_message(mes.chat_id(), "Send me a number.").await?;
        }
    }
    Ok(())
}

async fn handle_receive_location(
    bot: AutoSend<Bot>,
    mes: Message,
    dialogue: BotDialogue,
    state: ReceiveLocation,
) -> anyhow::Result<()> {
    let location = mes.text().unwrap();
    let message =
        format!("Full name: {}\nAge: {}\nLocation: {}", state.full_name, state.age, location);
    bot.send_message(mes.chat_id(), message).await?;
    dialogue.exit().await?;
    Ok(())
}
