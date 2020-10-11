use teloxide::{prelude::*};
use teloxide::contrib::managers::{StaticCommandParser, StaticCommandParserBuilder, DynamicCommandParser, DynamicCommandParserBuilder};
use teloxide::contrib::parser::{Parser, DataWithUWC};
use teloxide::contrib::handler::Handler;
use teloxide::contrib::callback::{Callback, Alternative};
use teloxide::teloxide as tlx;
use std::sync::Arc;

// ---------------- COMMAND help
#[derive(Parser)]
struct HelpCommandController {
    #[parser]
    parser: StaticCommandParser
}
impl HelpCommandController {
    pub fn init() -> Self {
        Self { 
            parser: StaticCommandParserBuilder::new("help").build()
        }
    }
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

#[derive(Parser)]
struct UsernameCommandController {
    #[parser]
    parser: DynamicCommandParser<Username>
}
impl UsernameCommandController {
    pub fn init() -> Self {
        Self {
            parser: DynamicCommandParserBuilder::new("username").build()
        }
    }
}
#[tlx(handler)]
async fn handle_username(_this: &UsernameCommandController, data: DataWithUWC<Username, Message>) -> Result<(), RequestError> {
    let DataWithUWC { data: username, uwc } = data;
    uwc.answer_str(format!("Your username is @{}.", username)).await?;
    Ok(())
}

// ---------------- COMMAND usernameandage

type UsernameAndAge = (String, u8);

#[derive(Parser)]
struct UsernameAndAgeCommandController {
    #[parser]
    parser: DynamicCommandParser<UsernameAndAge>
}
impl UsernameAndAgeCommandController {
    pub fn init() -> Self {
        Self {
            parser: DynamicCommandParserBuilder::new("usernameandage").build()
        }
    }
}
#[tlx(handler)]
async fn handle(_this: &UsernameAndAgeCommandController, data: DataWithUWC<UsernameAndAge, Message>) -> Result<(), RequestError> {
    let DataWithUWC { data, uwc } = data;
    uwc.answer_str(format!("Your username is @{} and age is {}.", data.0, data.1)).await?;
    Ok(())
}

// ---------------- SCHEMA
#[derive(Callback)]
struct CommandSchema {
    #[callback]
    handler: Alternative<
        HelpCommandController, 
        Alternative<
            UsernameCommandController, 
            UsernameAndAgeCommandController
        >
    >
}
impl CommandSchema {
    pub fn init() -> Self {
        let help = HelpCommandController::init();
        let username = UsernameCommandController::init();
        let username_and_age = UsernameAndAgeCommandController::init();
        Self {
            handler: Alternative::new(
                help,
                Alternative::new(
                    username,
                    username_and_age,
                )
            )
        }
    }
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env();
    
    let schema = Arc::new(CommandSchema::init());
    teloxide::repl(bot, move |upd| {
        let schema = schema.clone();
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
