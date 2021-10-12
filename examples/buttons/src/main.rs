use std::error::Error;
use teloxide::{
    payloads::SendMessageSetters,
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommand,
};

use tokio_stream::wrappers::UnboundedReceiverStream;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
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
async fn message_handler(
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

/// When it receives a callback from a button it edits the message with all
/// those buttons writing a text with the selected Debian version.
async fn callback_hander(
    cx: UpdateWithCx<AutoSend<Bot>, CallbackQuery>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let data = &cx.update.data;
    if let Some(text) = data {
        let callback: Vec<&str> = text.split('_').collect();
        let chat_id = callback[0];
        let version = callback[1];

        let message_id = cx.update.message.clone().unwrap().id;
        let _ = cx
            .requester
            .edit_message_text(chat_id.to_string(), message_id, format!("You chose: {}", version))
            .await;
        log::info!("You chose: {}", version);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                message_handler(cx).await.log_on_error().await;
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                callback_hander(cx).await.log_on_error().await;
            })
        })
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}
