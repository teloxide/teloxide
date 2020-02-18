use teloxide::{prelude::*, utils::command::BotCommand};

use futures::future;
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

fn generate() -> String {
    thread_rng().gen_range(0.0, 1.0).to_string()
}

async fn answer(
    cx: DispatcherHandlerCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Generate => cx.answer(generate()).send().await?,
        Command::Meow => cx.answer("I am a cat! Meow!").send().await?,
    };

    Ok(())
}

async fn handle_command(rx: DispatcherHandlerRx<Message>) {
    rx.filter_map(|cx| {
        future::ready(cx.update.text_owned().map(|text| (cx, text)))
    })
    .filter_map(|(cx, text)| {
        future::ready(Command::parse(&text).map(|(command, _)| (cx, command)))
    })
    .for_each_concurrent(None, |(cx, command)| async move {
        answer(cx, command).await.log_on_error().await;
    })
    .await;
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot).messages_handler(handle_command).dispatch().await;
}
