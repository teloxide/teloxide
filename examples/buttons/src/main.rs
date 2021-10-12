mod commands;

use std::error::Error;
use teloxide::prelude::*;

use tokio_stream::wrappers::UnboundedReceiverStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    teloxide::enable_logging!();
    log::info!("Starting bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, Message>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                commands::handler(cx).await.log_on_error().await;
            })
        })
        .callback_queries_handler(|rx: DispatcherHandlerRx<AutoSend<Bot>, CallbackQuery>| {
            UnboundedReceiverStream::new(rx).for_each_concurrent(None, |cx| async move {
                let data = &cx.update.data;
                // When it receives a callback from a button it edits the message with all those
                // buttons writing a text with the selected Debian version.
                if let Some(text) = data {
                    let callback: Vec<&str> = text.split('_').collect();
                    let chat_id = callback[0];
                    let version = callback[1];

                    let message_id = cx.update.message.clone().unwrap().id;
                    let _ = cx
                        .requester
                        .edit_message_text(
                            chat_id.to_string(),
                            message_id,
                            format!("You chose: {}", version),
                        )
                        .await;
                    log::info!("You chose: {}", version);
                }
            })
        })
        .dispatch()
        .await;

    log::info!("Closing bot... Goodbye!");

    Ok(())
}
