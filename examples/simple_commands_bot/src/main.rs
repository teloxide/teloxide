use teloxide::{prelude::*, utils::command::BotCommand};

use std::error::Error;
use std::sync::Arc;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    bot: Arc<AutoSend<Bot>>,
    message: Arc<Message>,
    command: Arc<Command>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command.as_ref() {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions()).await?
        }
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{}.", username)).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(message.chat.id, format!("Your username is @{} and age is {}.", username, age)).await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = "".into();// panic!("Your bot's name here");
    teloxide::commands_repl(bot, bot_name, answer, Command::ty()).await;
}
