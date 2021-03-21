use std::{error::Error, str::FromStr};

use teloxide::{prelude::*, utils::command::BotCommand};

use teloxide::types::ChatPermissions;

// Derive BotCommand to parse text with a command into this enumeration.
//
//  1. rename = "lowercase" turns all the commands into lowercase letters.
//  2. `description = "..."` specifies a text before all the commands.
//
// That is, you can just call Command::descriptions() to get a description of
// your commands in this format:
// %GENERAL-DESCRIPTION%
// %PREFIX%%COMMAND% - %DESCRIPTION%
#[derive(BotCommand)]
#[command(
    rename = "lowercase",
    description = "Use commands in format /%command% %num% %unit%",
    parse_with = "split"
)]
enum Command {
    #[command(description = "kick user from chat.")]
    Kick,
    #[command(description = "ban user in chat.")]
    Ban {
        time: u32,
        unit: UnitOfTime,
    },
    #[command(description = "mute user in chat.")]
    Mute {
        time: u32,
        unit: UnitOfTime,
    },
    Help,
}

enum UnitOfTime {
    Seconds,
    Minutes,
    Hours,
}

impl FromStr for UnitOfTime {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        match s {
            "h" | "hours" => Ok(UnitOfTime::Hours),
            "m" | "minutes" => Ok(UnitOfTime::Minutes),
            "s" | "seconds" => Ok(UnitOfTime::Seconds),
            _ => Err("Allowed units: h, m, s"),
        }
    }
}

// Calculates time of user restriction.
fn calc_restrict_time(time: u32, unit: UnitOfTime) -> u32 {
    match unit {
        UnitOfTime::Hours => time * 3600,
        UnitOfTime::Minutes => time * 60,
        UnitOfTime::Seconds => time,
    }
}

type Cx = UpdateWithCx<AutoSend<Bot>, Message>;

// Mute a user with a replied message.
async fn mute_user(cx: &Cx, time: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {
        Some(msg1) => {
            cx.requester
                .restrict_chat_member(
                    cx.update.chat_id(),
                    msg1.from().expect("Must be MessageKind::Common").id,
                    ChatPermissions::default(),
                )
                .until_date((cx.update.date + time as i32).try_into().unwrap())
                .await?;
        }
        None => {
            cx.reply_to("Use this command in reply to another message").send().await?;
        }
    }
    Ok(())
}

// Kick a user with a replied message.
async fn kick_user(cx: &Cx) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {
        Some(mes) => {
            // bot.unban_chat_member can also kicks a user from a group chat.
            cx.requester
                .unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id)
                .send()
                .await?;
        }
        None => {
            cx.reply_to("Use this command in reply to another message").send().await?;
        }
    }
    Ok(())
}

// Ban a user with replied message.
async fn ban_user(cx: &Cx, time: u32) -> Result<(), Box<dyn Error + Send + Sync>> {
    match cx.update.reply_to_message() {
        Some(message) => {
            cx.requester
                .kick_chat_member(
                    cx.update.chat_id(),
                    message.from().expect("Must be MessageKind::Common").id,
                )
                .until_date((cx.update.date + time as i32).try_into().unwrap())
                .await?;
        }
        None => {
            cx.reply_to("Use this command in a reply to another message!").send().await?;
        }
    }
    Ok(())
}

async fn action(cx: Cx, command: Command) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await.map(|_| ())?,
        Command::Kick => kick_user(&cx).await?,
        Command::Ban { time, unit } => ban_user(&cx, calc_restrict_time(time, unit)).await?,
        Command::Mute { time, unit } => mute_user(&cx, calc_restrict_time(time, unit)).await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting admin_bot...");

    let bot = Bot::from_env().auto_send();

    let bot_name: String = panic!("Your bot's name here");
    teloxide::commands_repl(bot, bot_name, action).await;
}
