use teloxide::{prelude::*, utils::command::BotCommand};

use rand::{thread_rng, Rng};

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "be a cat.")]
    Meow,
    #[command(description = "generate a random number within [0; 1).")]
    Generate,
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
        Command::Generate => {
            ctx.answer(thread_rng().gen_range(0.0, 1.0).to_string())
                .send()
                .await?
        }
        Command::Meow => ctx.answer("I am a cat! Meow!").send().await?,
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
