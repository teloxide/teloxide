use teloxide::{prelude::*, utils::command::BotCommand};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "be a cat.")]
    Meow,
    #[command(description = "be a dog.")]
    Woof,
    #[command(description = "be a cow.")]
    Moo,
}

async fn handle_command(
    ctx: DispatcherHandlerCtx<Message>,
) -> Result<(), RequestError> {
    let text = match ctx.update.text() {
        Some(text) => text,
        None => {
            log::info!("Received a message, but not text.");
            return Ok(());
        }
    };

    let command = match Command::parse(text) {
        Some((command, _)) => command,
        None => {
            log::info!("Received a text message, but not a command.");
            return Ok(());
        }
    };

    match command {
        Command::Help => ctx.answer(Command::descriptions()).send().await?,
        Command::Meow => ctx.answer("I am a cat! Meow!").send().await?,
        Command::Woof => ctx.answer("I am a dog! Woof!").send().await?,
        Command::Moo => ctx.answer("I am a cow! Moo!").send().await?,
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

    Dispatcher::<RequestError>::new(bot)
        .message_handler(&handle_command)
        .dispatch()
        .await;
}
