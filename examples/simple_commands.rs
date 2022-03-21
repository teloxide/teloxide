use teloxide::{prelude2::*, utils::command::BotCommand};

use std::error::Error;

#[derive(BotCommand, Clone)]
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
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => bot.send_message(message.chat.id, Command::descriptions()).await?,
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{}.", username)).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{} and age is {}.", username, age),
            )
            .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repls2::commands_repl(bot, answer, Command::ty()).await;
}
