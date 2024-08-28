//! This example demonstrates how to use deep linking in Telegram
//! by making a simple anonymous message bot.
//!
//! Deep linking (links like https://t.me/some_bot?start=123456789)
//! is handled by telegram in the same way as just sending /start {argument}.
//! So, in the StartCommand enum we need to write Start(String)
//! to get the argument, just like in command.rs example.
//!
//! Also, deep linking is only supported with /start command!
//! "https://t.me/some_bot?argument=123456789" will not work
//!
//! https://core.telegram.org/bots/features#deep-linking
use dptree::{case, deps};
use teloxide::{
    dispatching::dialogue::{self, InMemStorage},
    macros::BotCommands,
    prelude::*,
    types::{Me, ParseMode},
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum State {
    #[default]
    Start,
    WriteToSomeone {
        id: ChatId,
    },
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    Start(String),
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting deep linking bot...");

    let bot = Bot::from_env();

    let handler = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>()
                .branch(case![StartCommand::Start(start)].endpoint(start)),
        )
        .branch(
            Update::filter_message()
                .branch(case![State::WriteToSomeone { id }].endpoint(send_message)),
        );

    Dispatcher::builder(bot, handler)
        .dependencies(deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

pub async fn start(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message,
    start: String, // Available from `case![StartCommand::Start(start)]`
    me: Me,
) -> HandlerResult {
    if start.is_empty() {
        // This means that it is just a regular link like https://t.me/some_bot, or a /start command
        bot.send_message(
            msg.chat.id,
            format!(
                "Hello!\n\nThis link allows anyone to message you secretly: {}?start={}",
                me.tme_url(),
                msg.chat.id
            ),
        )
        .await?;
        dialogue.exit().await?;
    } else {
        // And this means that the link is like this: https://t.me/some_bot?start=123456789,
        // or a /start 123456789 command
        match start.parse::<i64>() {
            Ok(id) => {
                bot.send_message(msg.chat.id, "Send your message:").await?;
                dialogue.update(State::WriteToSomeone { id: ChatId(id) }).await?;
            }
            Err(_) => {
                bot.send_message(msg.chat.id, "Bad link!").await?;
                dialogue.exit().await?;
            }
        }
    }
    Ok(())
}

pub async fn send_message(
    bot: Bot,
    id: ChatId, // Available from `State::WriteToSomeone`
    msg: Message,
    dialogue: MyDialogue,
    me: Me,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            // Trying to send a message to the user
            let sent_result = bot
                .send_message(id, format!("You have a new message!\n\n<i>{text}</i>"))
                .parse_mode(ParseMode::Html)
                .await;

            // And if no error is returned, success!
            if sent_result.is_ok() {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Message sent!\n\nYour link is: {}?start={}",
                        me.tme_url(),
                        msg.chat.id
                    ),
                )
                .await?;
            } else {
                bot.send_message(msg.chat.id, "Error sending message! Maybe user blocked the bot?")
                    .await?;
            }
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "This bot can send only text.").await?;
        }
    };
    Ok(())
}
