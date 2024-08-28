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
    types::{ChatId, Message},
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
    ReceiveAge { full_name: String },
}

/// Helper struct to store only required information to answer messages and
/// reduce the size of the stack frames for handler functions
#[derive(Clone)]
struct IdsBundle {
    chat_id: ChatId,
    // Can be used to reply to messages or edit messages
    // message_id: MessageId,
}

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
    Update::filter_message()
        /*
           Currently the size of the `Message` struct (for TBA 6.9) is 1936 bytes, it's insane to copy it entirely in every handler's stack.
           So, here I introduce the `IdsBundle` which is 8 bytes in size, because all we need is a `chat_id`.
           The similar thing can be applied to the `CallbackQuery` struct which is
           even bigger..
           Take a look at this issue: https://github.com/teloxide/teloxide/issues/1118, maybe there will be
           more appropriate approach: `Arc<Message>` or similar.
        */
        .map(|msg: Message| IdsBundle { chat_id: msg.chat.id })
        .branch(
            Message::filter_text()
                .enter_dialogue::<Message, Storage, GlobalState>()
                .branch(
                    teloxide::filter_command::<Command, _>()
                        .branch(dptree::case![Command::Start].endpoint(ask_full_name)),
                )
                .branch(dptree::case![GlobalState::Idle].endpoint(handle_configured_user_message))
                .branch(
                    /*
                       Its essential not to use `dptree::case![GlobalState::UserSetup(UserSetup::ReceiveFullName)]` directly, this won't work.

                       Each nested enum requires it's own `branch` scope.
                       Actually, each `dptree::case![..]` introduces the inner enum value to the `DependencyMap`, so there is an option
                       to branch on the inner values freely.
                    */
                    dptree::case![GlobalState::UserSetup(_state)]
                        .branch(dptree::case![UserSetup::ReceiveFullName].endpoint(ask_age))
                        .branch(
                            dptree::case![UserSetup::ReceiveAge { full_name }]
                                .endpoint(finish_user_setup),
                        ),
                )
                .branch(dptree::endpoint(handle_unconfigured_user_message)),
        )
}

async fn ask_full_name(
    bot: Bot,
    dialogue: Dialogue,
    // For the sake of interest, take a look at the size of the `Message` struct
    IdsBundle { chat_id }: IdsBundle,
) -> HandlerResult {
    bot.send_message(chat_id, "Let's start! What's your full name?").await?;
    dialogue.update(GlobalState::UserSetup(UserSetup::ReceiveFullName)).await?;
    Ok(())
}

async fn ask_age(
    bot: Bot,
    dialogue: Dialogue,
    IdsBundle { chat_id }: IdsBundle,
    full_name: String,
) -> HandlerResult {
    bot.send_message(chat_id, format!("Hi, {full_name}! How old are you?")).await?;
    dialogue.update(GlobalState::UserSetup(UserSetup::ReceiveAge { full_name })).await?;
    Ok(())
}

async fn finish_user_setup(bot: Bot, dialogue: Dialogue, message: Message) -> HandlerResult {
    /*
       We did `Message::filter_text`, so it's safe to assume that this message contains text.
       Unfortunately, we can't get incoming text as the handler parameter, because it's
       shadowed by the `full_name` value from the `UserSetup::ReceiveAge {full_name}` state
    */
    let _age = match message.text().unwrap().parse::<u8>() {
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

async fn handle_configured_user_message(
    bot: Bot,
    IdsBundle { chat_id }: IdsBundle,
) -> HandlerResult {
    bot.send_message(chat_id, "Hi, configured user!").await?;
    Ok(())
}

async fn handle_unconfigured_user_message(
    bot: Bot,
    IdsBundle { chat_id }: IdsBundle,
) -> HandlerResult {
    bot.send_message(chat_id, "Use /start to setup your account").await?;
    Ok(())
}
