use std::sync::Arc;

use teloxide::{
    adaptors::throttle::Limits, dptree::di::DependencySupplier, prelude::*,
    utils::command::BotCommands,
};
use thiserror::Error;

#[derive(Debug, Error)]
enum PublicError {
    #[error("Hey, there's been a mistake!")]
    Dummy,
    #[error("Private error occured")]
    Private(#[from] PrivateError),
}

#[derive(Debug, Error)]
enum PrivateError {
    #[error("Teloxide request error: {0}")]
    Teloxide(#[from] teloxide::RequestError),
}

impl From<teloxide::RequestError> for PublicError {
    fn from(value: teloxide::RequestError) -> Self {
        Self::Private(PrivateError::Teloxide(value))
    }
}

#[derive(Clone, BotCommands)]
#[command(rename_rule = "snake_case")]
enum Command {
    Dummy,
}

type HandlerResult = Result<(), PublicError>;
type Bot = teloxide::adaptors::Throttle<teloxide::Bot>;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting error_handler example bot...");

    let bot = teloxide::Bot::from_env().throttle(Limits::default());

    let schema = dptree::entry().branch(
        Update::filter_message()
            .branch(
                dptree::entry()
                    .filter_command::<Command>()
                    .branch(dptree::case![Command::Dummy].endpoint(invoke_dummy_error)),
            )
            .branch(dptree::endpoint(say_hello)),
    );

    Dispatcher::builder(bot, schema)
        .error_handler(Arc::new(error_handler))
        .build()
        .dispatch()
        .await;
}

async fn error_handler(error: PublicError, deps: DependencyMap) {
    /*
        When an error is occured in one of your handlers, you can access only initial dependencies, such as:
        - Update
        - Bot
        - Me
        - and the ones you've provided to the Dispatcher's dependencies at the beginning
    */

    let bot: Arc<Bot> = deps.get();
    // It worth to note that in case of UpdateListener::Error there will be no
    // Update value
    let update: Arc<Update> = deps.get();
    let chat_id = update.chat().map(|c| c.id);

    match error {
        PublicError::Dummy => {
            // Some updates don't have a chat id
            if let Some(chat_id) = chat_id {
                // TODO, maybe retry queue? backon?
                let _ = bot.send_message(chat_id, error.to_string()).await;
            }
        }
        PublicError::Private(err) => match err {
            PrivateError::Teloxide(err) => {
                log::info!("Telegram API in unreachable: {err}");
            }
        },
    }
}

async fn invoke_dummy_error() -> HandlerResult {
    log::info!("This causes the Dummy error");
    Err(PublicError::Dummy)
}

async fn say_hello(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Hi!").await?;

    Ok(())
}
