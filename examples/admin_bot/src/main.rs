// TODO: simplify this and use typed command variants (see https://github.com/teloxide/teloxide/issues/152).

use teloxide::{
    prelude::*, types::ChatPermissions, utils::command::BotCommand,
};

use futures::future;

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
    description = "Use commands in format /%command% %num% %unit%"
)]
enum Command {
    #[command(description = "kick user from chat.")]
    Kick,
    #[command(description = "ban user in chat.")]
    Ban,
    #[command(description = "mute user in chat.")]
    Mute,

    Help,
}

// Calculates time of user restriction.
fn calc_restrict_time(num: i32, unit: &str) -> Result<i32, &str> {
    match unit {
        "h" | "hours" => Ok(num * 3600),
        "m" | "minutes" => Ok(num * 60),
        "s" | "seconds" => Ok(num),
        _ => Err("Allowed units: h, m, s"),
    }
}

// Parse arguments after a command.
fn parse_args(args: &[String]) -> Result<(i32, &str), &str> {
    let num = match args.get(0) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%"),
    };
    let unit = match args.get(1) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%"),
    };

    match num.parse::<i32>() {
        Ok(n) => Ok((n, unit)),
        Err(_) => Err("input positive number!"),
    }
}

// Parse arguments into a user restriction duration.
fn parse_time_restrict(args: &[String]) -> Result<i32, &str> {
    let (num, unit) = parse_args(args)?;
    calc_restrict_time(num, unit)
}

type Cx = DispatcherHandlerCx<Message>;

// Mute a user with a replied message.
async fn mute_user(cx: &Cx, args: &[String]) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(msg1) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                cx.bot
                    .restrict_chat_member(
                        cx.update.chat_id(),
                        msg1.from().expect("Must be MessageKind::Common").id,
                        ChatPermissions::default(),
                    )
                    .until_date(cx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                cx.bot
                    .restrict_chat_member(
                        cx.update.chat_id(),
                        msg1.from().unwrap().id,
                        ChatPermissions::default(),
                    )
                    .send()
                    .await?;
            }
        },
        None => {
            cx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}

// Kick a user with a replied message.
async fn kick_user(cx: &Cx) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(mes) => {
            // bot.unban_chat_member can also kicks a user from a group chat.
            cx.bot
                .unban_chat_member(cx.update.chat_id(), mes.from().unwrap().id)
                .send()
                .await?;
        }
        None => {
            cx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}

// Ban a user with replied message.
async fn ban_user(cx: &Cx, args: &[String]) -> ResponseResult<()> {
    match cx.update.reply_to_message() {
        Some(message) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                cx.bot
                    .kick_chat_member(
                        cx.update.chat_id(),
                        message.from().expect("Must be MessageKind::Common").id,
                    )
                    .until_date(cx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                cx.bot
                    .kick_chat_member(
                        cx.update.chat_id(),
                        message.from().unwrap().id,
                    )
                    .send()
                    .await?;
            }
        },
        None => {
            cx.reply_to("Use this command in a reply to another message!")
                .send()
                .await?;
        }
    }
    Ok(())
}

async fn action(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
    args: &[String],
) -> ResponseResult<()> {
    match command {
        Command::Help => {
            cx.answer(Command::descriptions()).send().await.map(|_| ())?
        }
        Command::Kick => kick_user(&cx).await?,
        Command::Ban => ban_user(&cx, args).await?,
        Command::Mute => mute_user(&cx, args).await?,
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    rx.filter(|cx| future::ready(cx.update.chat.is_group()))
        .commands::<Command, &str>(panic!("Insert here your bot's name"))
        .for_each_concurrent(None, |(cx, command, args)| async move {
            action(cx, command, &args).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting admin_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot).messages_handler(handle_commands).dispatch().await
}
