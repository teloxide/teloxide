> [v0.7 -> v0.8 migration guide >>](MIGRATION_GUIDE.md#07---08)

> `teloxide-core` versions less that `0.4.5` (`teloxide` versions less than 0.7.3) have a low-severity security vulnerability, [learn more >>](https://github.com/teloxide/teloxide/discussions/574)

<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  <a href="https://docs.rs/teloxide/">
    <img src="https://docs.rs/teloxide/badge.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/crates/v/teloxide.svg">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API%20coverage-Up%20to%205.7%20(inclusively)-green.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>

  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Highlights

 - **Declarative design.** teloxide is based upon [`dptree`], a functional-style [chain of responsibility] pattern that allows you to express pipelines of message processing in a highly declarative and extensible style.

[`dptree`]: https://github.com/p0lunin/dptree
[chain of responsibility]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern

 - **Dialogues management subsystem.** Our dialogues management subsystem is simple and easy-to-use, and, furthermore, is agnostic of how/where dialogues are stored. For example, you can just replace a one line to achieve [persistence]. Out-of-the-box storages include [Redis] and [Sqlite].

[persistence]: https://en.wikipedia.org/wiki/Persistence_(computer_science)
[Redis]: https://redis.io/
[Sqlite]: https://www.sqlite.org

 - **Strongly typed commands.** You can describe bot commands as enumerations, and then they'll be automatically constructed from strings — just like JSON structures in [`serde-json`] and command-line arguments in [`structopt`].

[`structopt`]: https://github.com/TeXitoi/structopt
[`serde-json`]: https://github.com/serde-rs/json

## Setting up your environment

 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows command line
$ set TELOXIDE_TOKEN=<Your token here>

# Windows PowerShell
$ $env:TELOXIDE_TOKEN=<Your token here>

```
 4. Make sure that your Rust compiler is up to date (teloxide currently requires rustc at least version 1.58):
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 5. Run `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = { version = "0.8", features = ["macros", "auto-send"] }
log = "0.4"
pretty_env_logger = "0.4"
tokio = { version =  "1.8", features = ["rt-multi-thread", "macros"] }
```

## API overview

### The dices bot

This bot replies with a dice throw to each received message:

([Full](examples/dices.rs))

```rust,no_run
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
        bot.send_dice(message.chat.id).await?;
        respond(())
    })
    .await;
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/dices.gif />
  </kbd>
</div>

### Commands

Commands are strongly typed and defined declaratively, similar to how we define CLI using [structopt] and JSON structures in [serde-json]. The following bot accepts these commands:

 - `/username <your username>`
 - `/usernameandage <your username> <your age>`
 - `/help`

[structopt]: https://docs.rs/structopt/0.3.9/structopt/
[serde-json]: https://github.com/serde-rs/json

([Full](examples/simple_commands.rs))

```rust,no_run
use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting simple_commands_bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "handle a username.")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        }
        Command::Username(username) => {
            bot.send_message(message.chat.id, format!("Your username is @{username}.")).await?
        }
        Command::UsernameAndAge { username, age } => {
            bot.send_message(
                message.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };

    Ok(())
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/simple-commands.gif />
  </kbd>
</div>

### Dialogues management

A dialogue is typically described by an enumeration where each variant is one possible state of the dialogue. There are also _state handler functions_, which may turn a dialogue from one state to another, thereby forming an [FSM].

[FSM]: https://en.wikipedia.org/wiki/Finite-state_machine

Below is a bot that asks you three questions and then sends the answers back to you:

([Full](examples/dialogue.rs))

```rust,ignore
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub enum State {
    Start,
    ReceiveFullName,
    ReceiveAge { full_name: String },
    ReceiveLocation { full_name: String, age: u8 },
}

impl Default for State {
    fn default() -> Self {
        Self::Start
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env().auto_send();

    Dispatcher::builder(
        bot,
        Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(teloxide::handler![State::Start].endpoint(start))
            .branch(teloxide::handler![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(teloxide::handler![State::ReceiveAge { full_name }].endpoint(receive_age))
            .branch(
                teloxide::handler![State::ReceiveLocation { full_name, age }]
                    .endpoint(receive_location),
            ),
    )
    .dependencies(dptree::deps![InMemStorage::<State>::new()])
    .build()
    .setup_ctrlc_handler()
    .dispatch()
    .await;
}

async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "Let's start! What's your full name?").await?;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

async fn receive_full_name(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
) -> HandlerResult {
    match msg.text() {
        Some(text) => {
            bot.send_message(msg.chat.id, "How old are you?").await?;
            dialogue.update(State::ReceiveAge { full_name: text.into() }).await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}

async fn receive_age(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    full_name: String, // Available from `State::ReceiveAge`.
) -> HandlerResult {
    match msg.text().map(|text| text.parse::<u8>()) {
        Some(Ok(age)) => {
            bot.send_message(msg.chat.id, "What's your location?").await?;
            dialogue.update(State::ReceiveLocation { full_name, age }).await?;
        }
        _ => {
            bot.send_message(msg.chat.id, "Send me a number.").await?;
        }
    }

    Ok(())
}

async fn receive_location(
    bot: AutoSend<Bot>,
    msg: Message,
    dialogue: MyDialogue,
    (full_name, age): (String, u8), // Available from `State::ReceiveLocation`.
) -> HandlerResult {
    match msg.text() {
        Some(location) => {
            let message = format!("Full name: {full_name}\nAge: {age}\nLocation: {location}");
            bot.send_message(msg.chat.id, message).await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(msg.chat.id, "Send me plain text.").await?;
        }
    }

    Ok(())
}
```

<div align="center">
  <kbd>
    <img src=../../raw/master/media/dialogue.gif />
  </kbd>
</div>

[More examples >>](examples/)

## FAQ

**Q: Where I can ask questions?**

A:

 - [Issues] is a good place for well-formed questions about the library design, enhancements, and bug reports.
 - [GitHub Discussions] is a place where you can ask us for help in a less formal manner.
 - If you need quick help in real-time, you should ask a question in [our official Telegram group].

[Issues]: https://github.com/teloxide/teloxide/issues
[our official Telegram group]: https://t.me/teloxide
[GitHub Discussions]: https://github.com/teloxide/teloxide/discussions

**Q: Do you support the Telegram API for clients?**

A: No, only the bots API.

**Q: Can I use webhooks?**

A: teloxide doesn't provide a special API for working with webhooks due to their nature with lots of subtle settings. Instead, you should setup your webhook by yourself, as shown in [`examples/ngrok_ping_pong_bot`](examples/ngrok_ping_pong.rs) and [`examples/heroku_ping_pong_bot`](examples/heroku_ping_pong.rs).

Associated links:
 - [Marvin's Marvellous Guide to All Things Webhook](https://core.telegram.org/bots/webhooks)
 - [Using self-signed certificates](https://core.telegram.org/bots/self-signed)

**Q: Can I handle both callback queries and messages within a single dialogue?**

A: Yes, see [`examples/purchase.rs`](examples/purchase.rs).

## Community bots

Feel free to propose your own bot to our collection!

 - [WaffleLapkin/crate_upd_bot](https://github.com/WaffleLapkin/crate_upd_bot) -- A bot that notifies about crate updates.
 - [mxseev/logram](https://github.com/mxseev/logram) -- Utility that takes logs from anywhere and sends them to Telegram.
 - [alexkonovalov/PedigreeBot](https://github.com/alexkonovalov/PedigreeBot) -- A Telegram bot for building family trees.
 - [Hermitter/tepe](https://github.com/Hermitter/tepe) -- A CLI to command a bot to send messages and files over Telegram.
 - [mattrighetti/GroupActivityBot](https://github.com/mattrighetti/group-activity-bot-rs) -- Telegram bot that keeps track of user activity in groups.
 - [mattrighetti/libgen-bot-rs](https://github.com/mattrighetti/libgen-bot-rs) -- Telgram bot to interface with libgen
 - [dracarys18/grpmr-rs](https://github.com/dracarys18/grpmr-rs) -- A Telegram group manager bot with variety of extra features.
 - [steadylearner/subreddit_reader](https://github.com/steadylearner/Rust-Full-Stack/tree/master/commits/teloxide/subreddit_reader) -- A bot that shows the latest posts at Rust subreddit.
 - [myblackbeard/basketball-betting-bot](https://github.com/myblackbeard/basketball-betting-bot) -- The bot lets you bet on NBA games against your buddies.
 - [ArtHome12/vzmuinebot](https://github.com/ArtHome12/vzmuinebot) -- Telegram bot for food menu navigate.
 - [ArtHome12/cognito_bot](https://github.com/ArtHome12/cognito_bot) -- The bot is designed to anonymize messages to a group.
 - [pro-vim/tg-vimhelpbot](https://github.com/pro-vim/tg-vimhelpbot) -- Link `:help` for Vim in Telegram.
 - [sschiz/janitor-bot](https://github.com/sschiz/janitor-bot) --  A bot that removes users trying to join to a chat that is designed for comments.
 - [slondr/BeerHolderBot](https://gitlab.com/slondr/BeerHolderBot) -- A bot that holds your beer.
 - [MustafaSalih1993/Miss-Vodka-Telegram-Bot](https://github.com/MustafaSalih1993/Miss-Vodka-Telegram-Bot) -- A Telegram bot written in rust using "Teloxide" library.
 - [x13a/tg-prompt](https://github.com/x13a/tg-prompt) -- Telegram prompt.
 - [magnickolas/remindee-bot](https://github.com/magnickolas/remindee-bot) -- Telegram bot for managing reminders.
 - [cyberknight777/knight-bot](https://gitlab.com/cyberknight777/knight-bot) -- A Telegram bot with variety of fun features.
 - [wa7sa34cx/the-black-box-bot](https://github.com/wa7sa34cx/the-black-box-bot) -- This is the Black Box Telegram bot. You can hold any items in it.
 - [crapstone/hsctt](https://codeberg.org/crapstones-bots/hsctt) -- A Telegram bot that searches for HTTP status codes in all messages and replies with the text form.
 - [alenpaul2001/AurSearchBot](https://gitlab.com/alenpaul2001/aursearchbot) -- Telegram bot for searching AUR in inline mode.
 - [studiedlist/EddieBot](https://gitlab.com/studiedlist/eddie-bot) -- Chatting bot with several entertainment features
  
## Contributing

See [`CONRIBUTING.md`](CONTRIBUTING.md).
