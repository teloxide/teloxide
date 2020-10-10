use teloxide::{prelude::*};
use teloxide::contrib::managers::{StaticCommandParser, StaticCommandParserBuilder, DynamicCommandParser, DynamicCommandParserBuilder};
use teloxide::contrib::parser::{Parser, DataWithUWC};
use teloxide::contrib::handler::Handler;
use teloxide::contrib::callback::{Callback, Alternative};

type ReqRes = Result<(), RequestError>;

// ---------------- COMMAND help
struct HelpCommandController {
    parser: StaticCommandParser
}
impl HelpCommandController {
    pub fn init() -> Self {
        Self { 
            parser: StaticCommandParserBuilder::new("help").build()
        }
    }
}
// TODO: proc-macro this
impl Parser for HelpCommandController {
    type Update = Message;
    type Output = ();

    fn parse(&self, data: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        self.parser.parse(data)
    }
}
#[async_trait::async_trait]
impl Handler for HelpCommandController {
    type Data = ();
    type Update = Message;
    type Err = RequestError;

    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) -> ReqRes {
        let DataWithUWC { data: _, uwc } = data;
        /* TODO: generating description using proc-macro */
        uwc.answer("help").send().await?;
        Ok(())
    }
}

// ---------------- COMMAND username

type Username = String;

struct UsernameCommandController {
    parser: DynamicCommandParser<Username>
}
impl UsernameCommandController {
    pub fn init() -> Self {
        Self {
            parser: DynamicCommandParserBuilder::new("username").build()
        }
    }
}
// TODO: proc-macro this
impl Parser for UsernameCommandController {
    type Update = Message;
    type Output = Username;

    fn parse(&self, data: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        self.parser.parse(data)
    }
}
#[async_trait::async_trait]
impl Handler for UsernameCommandController {
    type Data = Username;
    type Update = Message;
    type Err = RequestError;

    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) -> ReqRes {
        let DataWithUWC { data: username, uwc } = data;
        uwc.answer_str(format!("Your username is @{}.", username)).await?;
        Ok(())
    }
}

// ---------------- COMMAND usernameandage

type UsernameAndAge = (String, u8);

struct UsernameAndAgeCommandController {
    parser: DynamicCommandParser<UsernameAndAge>
}
impl UsernameAndAgeCommandController {
    pub fn init() -> Self {
        Self {
            parser: DynamicCommandParserBuilder::new("usernameandage").build()
        }
    }
}
// TODO: proc-macro this
impl Parser for UsernameAndAgeCommandController {
    type Update = Message;
    type Output = UsernameAndAge;

    fn parse(&self, data: UpdateWithCx<Self::Update>) -> Result<DataWithUWC<Self::Output, Self::Update>, UpdateWithCx<Self::Update>> {
        self.parser.parse(data)
    }
}
#[async_trait::async_trait]
impl Handler for UsernameAndAgeCommandController {
    type Data = UsernameAndAge;
    type Update = Message;
    type Err = RequestError;

    async fn handle(&self, data: DataWithUWC<Self::Data, Self::Update>) -> ReqRes {
        let DataWithUWC { data, uwc } = data;
        uwc.answer_str(format!("Your username is @{} and age is {}.", data.0, data.1)).await?;
        Ok(())
    }
}

// ---------------- SCHEMA
struct CommandSchema {
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
// TODO: proc-macro this
#[async_trait::async_trait]
impl Callback for CommandSchema {
    type Update = Message;
    type Err = RequestError;

    async fn try_handle(&self, input: UpdateWithCx<Self::Update>) -> Result<ReqRes, UpdateWithCx<Self::Update>> {
        self.handler.try_handle(input).await
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
    
    let schema = CommandSchema::init();
    // TODO: it's not work
    teloxide::repl(bot, |upd| async move { 
        match schema.try_handle(upd).await {
            Ok(res) => res,
            Err(cx) => {
                println!("Unhandled update: {:?}", cx);
                Ok(())
            }
        }
    }).await;
}
