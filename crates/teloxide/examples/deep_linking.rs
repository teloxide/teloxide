use dptree::{case, deps};
use teloxide::{
    dispatching::dialogue::{self, InMemStorage},
    macros::BotCommands,
    prelude::*,
    types::Me,
};

pub type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, PartialEq, Debug, Default)]
pub enum State {
    #[default]
    Start,
    WriteToSomeone {
        id: i64,
    },
}

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase")]
pub enum StartCommand {
    #[command()]
    Start(String), /* Because deep linking (links like https://t.me/some_bot?start=123456789)
                    * is the same as sending "/start 123456789",
                    * we can treat it as just an argument to a command
                    *
                    * https://core.telegram.org/bots/features#deep-linking */
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue bot...");

    let bot = Bot::from_env();

    let handler = dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(
            Update::filter_message()
                .filter_command::<StartCommand>() // Nessary to get cmd as an argument
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
    cmd: StartCommand,
    me: Me,
) -> HandlerResult {
    // If you have multiple commands, this will need to become a match
    let StartCommand::Start(arg) = cmd;

    if arg.is_empty() {
        // This means that it is just a regular link like https://t.me/some_bot, or a /start command
        bot.send_message(
            msg.chat.id,
            format!(
                "Hello!\n\nThis link allows anyone to message you secretly: {}?start={}",
                me.tme_url(),
                msg.chat.id.0
            ),
        )
        .await?;
        dialogue.exit().await?;
    } else {
        // And this means that the link is like this: https://t.me/some_bot?start=123456789
        match arg.parse::<i64>() {
            Ok(id) => {
                bot.send_message(msg.chat.id, "Send your message:").await?;
                dialogue.update(State::WriteToSomeone { id }).await?;
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
    id: i64, // Available from `State::WriteToSomeone`.
    msg: Message,
    dialogue: MyDialogue,
    me: Me,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            // Trying to send a message to the user
            let sent_result = bot
                .send_message(ChatId(id), format!("You have a new message!\n\n<i>{text}</i>"))
                .parse_mode(teloxide::types::ParseMode::Html)
                .await;

            // And if no error is returned, success!
            if sent_result.is_ok() {
                bot.send_message(
                    msg.chat.id,
                    format!(
                        "Message sent!\n\nYour link is: {}?start={}",
                        me.tme_url(),
                        msg.chat.id.0
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
