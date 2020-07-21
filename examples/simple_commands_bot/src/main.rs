use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand, Debug)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(
        description = "handle a username and an age.",
        parse_with = "split"
    )]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(cx: UpdateWithCx<(Message, Command)>) -> ResponseResult<()> {
    let command = &cx.update.1;

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Username(username) => {
            cx.answer_str(format!("Your username is @{}.", username)).await?
        }
        Command::UsernameAndAge { username, age } => {
            cx.answer_str(format!(
                "Your username is @{} and age is {}.",
                username, age
            ))
            .await?
        }
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .commands_handler::<_, &str>(
            |rx: DispatcherHandlerRx<(Message, Command)>| {
                rx.for_each_concurrent(None, |update| async move {
                    answer(update).await.log_on_error().await;
                })
            },
            panic!("Your bot's name"),
        )
        .dispatch()
        .await;
}
