use teloxide::{prelude::*, Cascade};
use teloxide::contrib::managers::{StaticCommandParser, DynamicCommandParser};
use teloxide::contrib::parser::{Parser, DataWithUWC};
use teloxide::contrib::handler::Handler;
use teloxide::contrib::callback::Callback;
use teloxide::teloxide as tlx;
use teloxide::teloc::{Dependency, Resolver};
use teloxide::teloc;
use std::sync::Arc;
use teloc::ServiceProvider;

// ---------------- COMMAND help
#[derive(Parser, Dependency)]
struct HelpCommandController {
    #[parser]
    #[init("/help")]
    parser: StaticCommandParser
}
#[tlx(handler)]
async fn handle_help(_this: &HelpCommandController, data: DataWithUWC<(), Message>) -> Result<(), RequestError> {
    let DataWithUWC { data: _, uwc } = data;
    /* TODO: generating description using proc-macro */
    uwc.answer("help").send().await?;
    Ok(())
}

// ---------------- COMMAND username

type Username = String;

#[derive(Parser, Dependency)]
struct UsernameCommandController {
    #[parser]
    #[init("/username", " ")]
    parser: DynamicCommandParser<Username>
}
#[tlx(handler)]
async fn handle_username(_this: &UsernameCommandController, data: DataWithUWC<Username, Message>) -> Result<(), RequestError> {
    let DataWithUWC { data: username, uwc } = data;
    uwc.answer_str(format!("Your username is @{}.", username)).await?;
    Ok(())
}

// ---------------- COMMAND usernameandage

type UsernameAndAge = (String, u8);

#[derive(Parser, Dependency)]
struct UsernameAndAgeCommandController {
    #[parser]
    #[init("usernameandage", " ")]
    parser: DynamicCommandParser<UsernameAndAge>
}
#[tlx(handler)]
async fn handle(_this: &UsernameAndAgeCommandController, data: DataWithUWC<UsernameAndAge, Message>) -> Result<(), RequestError> {
    let DataWithUWC { data, uwc } = data;
    uwc.answer_str(format!("Your username is @{} and age is {}.", data.0, data.1)).await?;
    Ok(())
}

// ---------------- SCHEMA
#[derive(Callback, Dependency)]
struct CommandSchema {
    #[callback]
    handler: Cascade![
        HelpCommandController, 
        UsernameCommandController, 
        UsernameAndAgeCommandController
    ]
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env();
    
    let container = ServiceProvider::new()
        .add_transient::<HelpCommandController>()
        .add_transient::<UsernameCommandController>()
        .add_transient::<UsernameAndAgeCommandController>()
        .add_transient::<CommandSchema>();
    let container = Arc::new(container);
    
    teloxide::repl(bot, move |upd| {
        let schema: CommandSchema = container.scope_().resolve();
        async move {
            match schema.try_handle(upd).await {
                Ok(res) => res,
                Err(cx) => {
                    println!("Unhandled update: {:?}", cx);
                    Ok(())
                }
            }
        }
    }).await;
}
