use teloxide::prelude::*;
use teloxide::utils::command::BotCommand;
use teloxide::types::ChatPermissions;

type Ctx = DispatcherHandlerCtx<Message>;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "Use commands in format /%command% %num% %unit%")]
enum Command {
    #[command(description = "kick user from chat.")]
    Kick,
    #[command(description = "ban user in chat.")]
    Ban,
    #[command(description = "mute user in chat.")]
    Mute,

    Help,
}

fn calc_restrict_time(num: i32, unit: &str) -> Result<i32, &str> {
    match unit {
        "h"|"hours" => Ok(num * 3600),
        "m"|"minutes" => Ok(num * 60),
        "s"|"seconds" => Ok(num),
        _ => Err("allowed units: h, m, s")
    }
}

fn parse_args(args: Vec<&str>) -> Result<(i32, &str), &str> {
    let num = match args.get(0) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%"),
    };
    let unit = match args.get(1) {
        Some(s) => s,
        None => return Err("Use command in format /%command% %num% %unit%")
    };

    match num.parse::<i32>() {
        Ok(n) => Ok((n, unit)),
        Err(_) => Err("input positive number!"),
    }
}

fn parse_time_restrict(args: Vec<&str>) -> Result<i32, &str> {
    let (num, unit) = parse_args(args)?;
    calc_restrict_time(num, unit)
}

async fn handle_command(ctx: Ctx) -> Result<(), ()> {
    if let Some(text) = ctx.update.text() {
        let (command, args): (Command, Vec<&str>) = Command::parse(text).ok_or(())?;

        match command {
            Command::Help => {
                ctx.answer(Command::descriptions()).send().await;
            }
            Command::Kick => {
                match ctx.update.reply_to_message() {
                    Some(mes) => {
                        if let Some(user) = mes.from() {
                            ctx.bot.unban_chat_member(
                                ctx.update.chat_id(),
                                user.id
                            ).send().await;
                        }
                    },
                    None => {
                        ctx.reply_to("Use this command in reply to another message").send().await;
                    }
                }
            }
            Command::Ban => {
                match ctx.update.reply_to_message() {
                    Some(mes) => match parse_time_restrict(args) {
                        Ok(time) => {
                            if let Some(user) = mes.from() {
                                ctx.bot.kick_chat_member(
                                    ctx.update.chat_id(),
                                    user.id
                                )
                                    .until_date(ctx.update.date + time)
                                    .send()
                                    .await;
                            }
                        }
                        Err(msg) => {
                            ctx.answer(msg).send().await;
                        },
                    },
                    None => {
                        ctx.reply_to("Use this command in reply to another message").send().await;
                    },
                }
            }
            Command::Mute => {
                match ctx.update.reply_to_message() {
                    Some(mes) => match parse_time_restrict(args) {
                        Ok(time) => {
                            if let Some(user) = mes.from() {
                                ctx.bot.restrict_chat_member(
                                ctx.update.chat_id(),
                                user.id,
                                ChatPermissions::default()
                            )
                                .until_date(ctx.update.date + time)
                                .send()
                                .await;
                            }
                        }
                        Err(msg) => {
                            ctx.answer(msg).send().await;
                        }
                    },
                    None => {
                        ctx.reply_to("Use this command in reply to another message").send().await;
                    },
                }
            }
        };
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let bot = Bot::new("YourAwesomeToken");
    Dispatcher::new(bot)
        .message_handler(&handle_command)
        .dispatch()
        .await
}
