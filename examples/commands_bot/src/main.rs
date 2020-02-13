use teloxide::{
    prelude::*, types::ChatPermissions, utils::command::BotCommand,
};

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
fn parse_args(args: Vec<&str>) -> Result<(i32, &str), &str> {
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
fn parse_time_restrict(args: Vec<&str>) -> Result<i32, &str> {
    let (num, unit) = parse_args(args)?;
    calc_restrict_time(num, unit)
}

type Ctx = DispatcherHandlerCtx<Message>;

// Mute a user with a replied message.
async fn mute_user(ctx: &Ctx, args: Vec<&str>) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(msg1) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                ctx.bot
                    .restrict_chat_member(
                        ctx.update.chat_id(),
                        msg1.from().expect("Must be MessageKind::Common").id,
                        ChatPermissions::default(),
                    )
                    .until_date(ctx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                ctx.bot
                    .restrict_chat_member(
                        ctx.update.chat_id(),
                        msg1.from().unwrap().id,
                        ChatPermissions::default(),
                    )
                    .send()
                    .await?;
            }
        },
        None => {
            ctx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}

// Kick a user with a replied message.
async fn kick_user(ctx: &Ctx) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(mes) => {
            // bot.unban_chat_member can also kicks a user from a group chat.
            ctx.bot
                .unban_chat_member(ctx.update.chat_id(), mes.from().unwrap().id)
                .send()
                .await?;
        }
        None => {
            ctx.reply_to("Use this command in reply to another message")
                .send()
                .await?;
        }
    }
    Ok(())
}

// Ban a user with replied message.
async fn ban_user(ctx: &Ctx, args: Vec<&str>) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(message) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                ctx.bot
                    .kick_chat_member(
                        ctx.update.chat_id(),
                        message.from().expect("Must be MessageKind::Common").id,
                    )
                    .until_date(ctx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(_) => {
                ctx.bot
                    .kick_chat_member(
                        ctx.update.chat_id(),
                        message.from().unwrap().id,
                    )
                    .send()
                    .await?;
            }
        },
        None => {
            ctx.reply_to("Use this command in a reply to another message!")
                .send()
                .await?;
        }
    }
    Ok(())
}

// Handle all messages.
async fn handle_command(ctx: Ctx) -> Result<(), RequestError> {
    if ctx.update.chat.is_group() {
        // The same as DispatcherHandlerResult::exit(Ok(())). If you have more
        // handlers, use DispatcherHandlerResult::next(...)
        return Ok(());
    }

    if let Some(text) = ctx.update.text() {
        // Parse text into a command with args.
        let (command, args): (Command, Vec<&str>) = match Command::parse(text) {
            Some(tuple) => tuple,
            None => return Ok(()),
        };

        match command {
            Command::Help => {
                ctx.answer(Command::descriptions()).send().await?;
            }
            Command::Kick => {
                kick_user(&ctx).await?;
            }
            Command::Ban => {
                ban_user(&ctx, args).await?;
            }
            Command::Mute => {
                mute_user(&ctx, args).await?;
            }
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting commands_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .message_handler(&handle_command)
        .dispatch()
        .await
}
