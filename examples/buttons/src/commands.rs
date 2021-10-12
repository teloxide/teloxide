use std::error::Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::{AutoSend, Bot, GetChatId, Message, UpdateWithCx},
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommand,
};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
pub enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Start")]
    Start,
}

/// Creates a keyboard made by buttons in a big column.
fn make_keyboard(chat_id: i64) -> InlineKeyboardMarkup {
    let mut keyboard_array: Vec<Vec<InlineKeyboardButton>> = vec![];
    // The column is made by the list of Debian versions.
    let debian_versions = vec![
        "Buzz", "Rex", "Bo", "Hamm", "Slink", "Potato", "Woody", "Sarge", "Etch", "Lenny",
        "Squeeze", "Wheezy", "Jessie", "Stretch", "Buster", "Bullseye",
    ];

    for version in debian_versions {
        // Match each button with the chat id and the Debian version.
        keyboard_array.push(vec![InlineKeyboardButton::callback(
            version.into(),
            format!("{}_{}", chat_id, version),
        )]);
    }

    InlineKeyboardMarkup::new(keyboard_array)
}

/// Parse the text wrote on Telegram and check if that text is a valid command
/// or not, then match the command. If the command is `/start` it writes a
/// markup with the `InlineKeyboardMarkup`.
pub async fn handler(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Ok(command) =
        BotCommand::parse(cx.update.text().expect("Error with the text"), "buttons")
    {
        match command {
            Command::Help => {
                // Just send the description of all commands.
                cx.answer(Command::descriptions()).await?;
            }
            Command::Start => {
                let keyboard = make_keyboard(cx.chat_id());
                // Create a list of buttons using callbacks to receive the response.
                cx.answer("Debian versions:").reply_markup(keyboard).await?;
            }
        }
    } else {
        cx.reply_to("Command not found!").await?;
    }

    Ok(())
}
