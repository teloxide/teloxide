//! Imagine that your dialogue state is logically represented by separate
//! stages, say "setup stage", "perform action stage", etc. Instead of inflating
//! a single-state enumeration like this:
//! ```
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Unconfigured,
//!     ReceiveFullName,
//!     ReceiveAge {
//!         full_name: String,
//!     },
//!     // Many more variants...
//!     Idle,
//! }
//! ```
//!
//! The more appropriate way is to nest enumerations like this:
//! ```
//! #[derive(Clone, Default)]
//! pub enum GlobalState {
//!     #[default]
//!     Unconfigured,
//!     UserSetup(UserSetup),
//!     // Many more complex stages...
//!     Idle,
//! }
//!
//! #[derive(Clone)]
//! enum UserSetup {
//!     ReceiveFullName,
//!     ReceiveAge { full_name: String },
//! }
//!
//! // More enumeration definitions...
//! ```
//!
//! This example demonstrates how to achieve this `teloxide` design pattern.

use teloxide::{
    dispatching::{dialogue::InMemStorage, MessageFilterExt},
    prelude::*,
    types::Message,
};

type Bot = teloxide::Bot;
type Error = Box<dyn std::error::Error + Send + Sync>;
type HandlerResult = Result<(), Error>;
type UpdateHandler = teloxide::dispatching::UpdateHandler<Error>;
type Storage = InMemStorage<GlobalState>;
type Dialogue = teloxide::dispatching::dialogue::Dialogue<GlobalState, Storage>;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "snake_case")]
enum Command {
    #[command(description = "Setup user account")]
    Start,
}

#[derive(Clone, Default)]
enum GlobalState {
    #[default]
    Unconfigured,
    UserSetup(UserSetup),
    Idle,
}

#[derive(Clone)]
enum UserSetup {
    ReceiveFullName,
    ReceiveAge { full_name: FullName },
}

#[derive(Clone, derive_more::Display)]
struct FullName(pub String);

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting the \"composite_state\" example");

    let bot = Bot::from_env();

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<GlobalState>::new()])
        .build()
        .dispatch()
        .await
}

fn schema() -> UpdateHandler {
    Update::filter_message().branch(
        Message::filter_text()
            .enter_dialogue::<Message, Storage, GlobalState>()
            .branch(
                teloxide::filter_command::<Command, _>()
                    .branch(dptree::case![Command::Start].endpoint(ask_full_name)),
            )
            .branch(dptree::case![GlobalState::Idle].endpoint(handle_configured_user_message))
            .branch(
                // Its essential not to use
                // `dptree::case![GlobalState::UserSetup(UserSetup::ReceiveFullName)]` directly,
                // this won't work. Each nested enum requires it's own `branch`
                // scope. Actually, each `dptree::case![..]` introduces the inner
                // enum value to the `DependencyMap`, so there is an option
                // to branch on the inner values freely.
                dptree::case![GlobalState::UserSetup(_state)]
                    .branch(
                        dptree::case![UserSetup::ReceiveFullName]
                            .map(|text: String| FullName(text))
                            .endpoint(ask_age),
                    )
                    .branch(
                        dptree::case![UserSetup::ReceiveAge { full_name }]
                            .endpoint(finish_user_setup),
                    ),
            )
            .branch(dptree::endpoint(handle_unconfigured_user_message)),
    )
}

async fn ask_full_name(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Let's start! What's your full name?").await?;
    dialogue.update(GlobalState::UserSetup(UserSetup::ReceiveFullName)).await?;
    Ok(())
}

async fn ask_age(
    bot: Bot,
    dialogue: Dialogue,
    message: Message,
    full_name: FullName,
) -> HandlerResult {
    bot.send_message(message.chat.id, format!("Hi, {full_name}! How old are you?")).await?;
    dialogue.update(GlobalState::UserSetup(UserSetup::ReceiveAge { full_name })).await?;
    Ok(())
}

async fn finish_user_setup(
    bot: Bot,
    dialogue: Dialogue,
    message: Message,
    age: String,
) -> HandlerResult {
    let _age = match age.parse::<u8>() {
        Ok(age) => age,
        Err(_err) => {
            bot.send_message(message.chat.id, "Please, enter your age").await?;
            return Ok(());
        }
    };

    bot.send_message(message.chat.id, "You've successfully passed setup").await?;
    dialogue.update(GlobalState::Idle).await?;
    Ok(())
}

async fn handle_configured_user_message(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Hi, configured user!").await?;
    Ok(())
}

async fn handle_unconfigured_user_message(bot: Bot, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Use /start to setup your account").await?;
    Ok(())
}
