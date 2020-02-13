use teloxide::{
    prelude::*, types::ChatPermissions, utils::command::BotCommand,
};

// Declare type of handler context
type Ctx = DispatcherHandlerCtx<Message>;

// Derive trait which allow to parse text with command into enum
// (rename = "lowercase") means that names of variants of enum will be lowercase
// before parsing `description` will be add before description of command when
// you call Command::descriptions()
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

// Calculate time of restrict user.
fn calc_restrict_time(num: i32, unit: &str) -> Result<i32, &str> {
    match unit {
        "h" | "hours" => Ok(num * 3600),
        "m" | "minutes" => Ok(num * 60),
        "s" | "seconds" => Ok(num),
        _ => Err("Allowed units: h, m, s"),
    }
}

// Parse args which user printed after command.
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

// Parse input args into time to restrict
fn parse_time_restrict(args: Vec<&str>) -> Result<i32, &str> {
    let (num, unit) = parse_args(args)?;
    calc_restrict_time(num, unit)
}

// Mute user by replied message
async fn mute_user(ctx: &Ctx, args: Vec<&str>) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(mes) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                ctx.bot
                    .restrict_chat_member(
                        ctx.update.chat_id(),
                        // Sender of message cannot be only in messages from
                        // channels so we can use
                        // unwrap()
                        mes.from().unwrap().id,
                        ChatPermissions::default(),
                    )
                    .until_date(ctx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(msg) => {
                ctx.bot
                    .restrict_chat_member(
                        ctx.update.chat_id(),
                        mes.from().unwrap().id,
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

// Kick user by replied message
async fn kick_user(ctx: &Ctx) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(mes) => {
            // `unban_chat_member` will also kick user from group chat
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

// Ban user by replied message
async fn ban_user(ctx: &Ctx, args: Vec<&str>) -> Result<(), RequestError> {
    match ctx.update.reply_to_message() {
        Some(mes) => match parse_time_restrict(args) {
            // Mute user temporarily...
            Ok(time) => {
                ctx.bot
                    .kick_chat_member(
                        ctx.update.chat_id(),
                        mes.from().unwrap().id,
                    )
                    .until_date(ctx.update.date + time)
                    .send()
                    .await?;
            }
            // ...or permanently
            Err(msg) => {
                ctx.bot
                    .kick_chat_member(
                        ctx.update.chat_id(),
                        mes.from().unwrap().id,
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

// Handle all messages
async fn handle_command(ctx: Ctx) -> Result<(), RequestError> {
    // If message not from group stop handled.
    // NOTE: in this case we have only one `message_handler`. If you have more,
    // return DispatcherHandlerResult::next() so that the following handlers
    // can receive this message!
    if ctx.update.chat.is_group() {
        return Ok(());
    }

    if let Some(text) = ctx.update.text() {
        // Parse text into command with args
        let (command, args): (Command, Vec<&str>) = match Command::parse(text) {
            Some(tuple) => tuple,
            None => return Ok(()),
        };

        match command {
            Command::Help => {
                // Command::descriptions() return a message in format:
                //
                // %general_description%
                // %prefix%%command% - %description%
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
