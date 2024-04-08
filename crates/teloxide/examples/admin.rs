use std::str::FromStr;

use chrono::Duration;
use teloxide::{prelude::*, types::ChatPermissions, utils::command::BotCommands};

// Derive BotCommands to parse text with a command into this enumeration.
//
// 1. `rename_rule = "lowercase"` turns all the commands into lowercase letters.
// 2. `description = "..."` specifies a text before all the commands.
//
// That is, you can just call Command::descriptions() to get a description of
// your commands in this format:
// %GENERAL-DESCRIPTION%
// %PREFIX%%COMMAND% - %DESCRIPTION%

/// Use commands in format /%command% %num% %unit%
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", parse_with = "split")]
enum Command {
    /// Kick user from chat.
    Kick,
    /// Ban user in chat.
    Ban {
        time: u64,
        unit: UnitOfTime,
    },
    /// Mute user in chat.
    Mute {
        time: u64,
        unit: UnitOfTime,
    },
    Help,
}

#[derive(Clone)]
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

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting admin bot...");

    let bot = teloxide::Bot::from_env();

    Command::repl(bot, action).await;
}

async fn action(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::Kick => kick_user(bot, msg).await?,
        Command::Ban { time, unit } => ban_user(bot, msg, calc_restrict_time(time, unit)).await?,
        Command::Mute { time, unit } => mute_user(bot, msg, calc_restrict_time(time, unit)).await?,
    };

    Ok(())
}

// Kick a user with a replied message.
async fn kick_user(bot: Bot, msg: Message) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            // bot.unban_chat_member can also kicks a user from a group chat.
            bot.unban_chat_member(msg.chat.id, replied.from().unwrap().id).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Use this command in reply to another message").await?;
        }
    }
    Ok(())
}

// Ban a user with replied message.
async fn ban_user(bot: Bot, msg: Message, time: Duration) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.kick_chat_member(
                msg.chat.id,
                replied.from().expect("Must be MessageKind::Common").id,
            )
            .until_date(msg.date + time)
            .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Use this command in a reply to another message!")
                .await?;
        }
    }
    Ok(())
}

// Mute a user with a replied message.
async fn mute_user(bot: Bot, msg: Message, time: Duration) -> ResponseResult<()> {
    match msg.reply_to_message() {
        Some(replied) => {
            bot.restrict_chat_member(
                msg.chat.id,
                replied.from().expect("Must be MessageKind::Common").id,
                ChatPermissions::empty(),
            )
            .until_date(msg.date + time)
            .await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Use this command in a reply to another message!")
                .await?;
        }
    }
    Ok(())
}

// Calculates time of user restriction.
fn calc_restrict_time(time: u64, unit: UnitOfTime) -> Duration {
    // FIXME: actually handle the case of too big integers correctly, instead of
    // unwrapping
    match unit {
        UnitOfTime::Hours => Duration::try_hours(time as i64).unwrap(),
        UnitOfTime::Minutes => Duration::try_minutes(time as i64).unwrap(),
        UnitOfTime::Seconds => Duration::try_seconds(time as i64).unwrap(),
    }
}
