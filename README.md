<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  
  <a href="https://docs.rs/teloxide/">
    <img src="https://img.shields.io/badge/docs.rs-v0.2.0-blue.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/badge/crates.io-v0.2.0-orange.svg">
  </a>
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>
  <a href="https://core.telegram.org/bots/api">
    <img src="https://img.shields.io/badge/API coverage-Up to 0.4.6 (inclusively)-green.svg">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Table of contents
 - [Features](https://github.com/teloxide/teloxide#features)
 - [Setting up your environment](https://github.com/teloxide/teloxide#setting-up-your-environment)
 - [API overview](https://github.com/teloxide/teloxide#api-overview)
   - [The ping-pong bot](https://github.com/teloxide/teloxide#the-ping-pong-bot)
   - [Commands](https://github.com/teloxide/teloxide#commands)
   - [Dialogues](https://github.com/teloxide/teloxide#dialogues)
 - [Recommendations](https://github.com/teloxide/teloxide#recommendations)
 - [FAQ](https://github.com/teloxide/teloxide#faq)
 - [Community bots](https://github.com/teloxide/teloxide#community-bots)
 - [Contributing](https://github.com/teloxide/teloxide#contributing)

## Features

 - **Functioal reactive design.** teloxide has [functional reactive design], allowing you to declaratively manipulate streams of updates from Telegram using filters, maps, folds, zips, and a lot of [other adaptors].

[functional reactive design]: https://en.wikipedia.org/wiki/Functional_reactive_programming
[other adaptors]: https://docs.rs/futures/latest/futures/stream/trait.StreamExt.html

 - **Persistence.** Dialogues management is independent of how/where dialogues are stored: you can just replace one line and make them [persistent]. Out-of-the-box storages include [Redis].

[persistent]: https://en.wikipedia.org/wiki/Persistence_(computer_science)
[Redis]: https://redis.io/

 - **Strongly typed bot commands.** You can describe bot commands as enumerations, and then they'll be automatically constructed from strings. Just like you describe JSON structures in [serde-json] and command-line arguments in [structopt].

[structopt]: https://github.com/TeXitoi/structopt
[serde-json]: https://github.com/serde-rs/json

## Setting up your environment
 1. [Download Rust](http://rustup.rs/).
 2. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 3. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix-like
$ export TELOXIDE_TOKEN=<Your token here>

# Windows
$ set TELOXIDE_TOKEN=<Your token here>
```
 4. Be sure that you are up to date:
```bash
# If you're using stable
$ rustup update stable
$ rustup override set stable

# If you're using nightly
$ rustup update nightly
$ rustup override set nightly
```

 5. Execute `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = "0.2.0"
log = "0.4.8"
tokio = "0.2.11"
pretty_env_logger = "0.4.0"
```

## API overview

### The ping-pong bot
This bot has a single message handler, which answers "pong" to each incoming message:

([Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/src/main.rs))
```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.answer("pong").send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}

```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/PING_PONG_BOT.png width="600" />
  </kbd>
</div>

### Commands
Commands are defined similar to how we define CLI using [structopt] and JSON structures in [serde-json]. The following bot accepts either `/username YourUsername`, `/usernameandage YourUsername YourAge` and shows the usage guide on `/help`:

[structopt]: https://docs.rs/structopt/0.3.9/structopt/
[serde-json]: https://github.com/serde-rs/json

([Full](https://github.com/teloxide/teloxide/blob/master/examples/simple_commands_bot/src/main.rs))
```rust
// Imports are omitted...

#[derive(BotCommand)]
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

async fn answer(
    cx: UpdateWithCx<Message>,
    command: Command,
) -> ResponseResult<()> {
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

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    rx.commands::<Command, &str>(panic!("Insert here your bot's name"))
        .for_each_concurrent(None, |(cx, command)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    // Setup is omitted...
}
```

### Dialogues
A dialogue is described by an enumeration, where each variant is one of possible dialogue's states. There are also _transition functions_, which turn a dialogue from one state to another, thereby forming an [FSM].

[FSM]: https://en.wikipedia.org/wiki/Finite-state_machine

States and transition functions are placed into separated modules. For example:

([dialogue_bot/src/states.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/states.rs))
```rust
// Imports are omitted...

#[derive(BotDialogue, SmartDefault, From)]
pub enum Dialogue {
    #[default]
    #[transition(start)]
    Start(StartState),

    #[transition(receive_days_of_week)]
    ReceiveDaysOfWeek(ReceiveDaysOfWeekState),

    #[transition(receive_10x5_answer)]
    Receive10x5Answer(Receive10x5AnswerState),

    #[transition(receive_gandalf_alternative_name)]
    ReceiveGandalfAlternativeName(ReceiveGandalfAlternativeNameState),
}

#[derive(Default)]
pub struct StartState;

pub struct ReceiveDaysOfWeekState {
    rest: StartState,
}

pub struct Receive10x5AnswerState {
    rest: ReceiveDaysOfWeekState,
    days_of_week: u8,
}

pub struct ReceiveGandalfAlternativeNameState {
    rest: Receive10x5AnswerState,
    _10x5_answer: u8,
}

pub struct ExitState {
    rest: ReceiveGandalfAlternativeNameState,
    gandalf_alternative_name: String,
}

up!(
    StartState -> ReceiveDaysOfWeekState,
    ReceiveDaysOfWeekState + [days_of_week: u8] -> Receive10x5AnswerState,
    Receive10x5AnswerState + [_10x5_answer: u8] -> ReceiveGandalfAlternativeNameState,
    ReceiveGandalfAlternativeNameState + [gandalf_alternative_name: String] -> ExitState,
);
```

The handy `up!` macro automatically generates functions that complete one state to another by appending a field. Here are the transition functions:

([dialogue_bot/src/transitions.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/transitions.rs))
```rust
// Imports are omitted...

pub type Out = TransitionOut<Dialogue>;

pub async fn start(cx: TransitionIn, state: StartState) -> Out {
    cx.answer_str("Let's start our test! How many days per week are there?")
        .await?;
    next(state.up())
}

pub async fn receive_days_of_week(
    cx: TransitionIn,
    state: ReceiveDaysOfWeekState,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 7 => {
            cx.answer_str("10*5 = ?").await?;
            next(state.up(ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

pub async fn receive_10x5_answer(
    cx: TransitionIn,
    state: Receive10x5AnswerState,
) -> Out {
    match cx.update.text().map(str::parse) {
        Some(Ok(ans)) if ans == 50 => {
            cx.answer_str("What's an alternative name of Gandalf?").await?;
            next(state.up(ans))
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}

pub async fn receive_gandalf_alternative_name(
    cx: TransitionIn,
    state: ReceiveGandalfAlternativeNameState,
) -> Out {
    match cx.update.text() {
        Some(ans) if ans == "Mithrandir" => {
            cx.answer_str(
                "Congratulations! You've successfully passed the test!",
            )
            .await?;
            exit()
        }
        _ => {
            cx.answer_str("Try again.").await?;
            next(state)
        }
    }
}
```

([dialogue_bot/src/main.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/main.rs))
```rust
// Imports are omitted...

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |input: DialogueWithCx<Message, Dialogue, Infallible>| async move {
                // Unwrap without panic because of std::convert::Infallible.
                input
                    .dialogue
                    .unwrap()
                    .dispatch(input.cx)
                    .await
                    .expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}
```

[More examples!](https://github.com/teloxide/teloxide/tree/master/examples)

## Recommendations
 - Use this pattern:
 
 ```rust
 #[tokio::main]
 async fn main() {
     run().await;
 }
 
 async fn run() {
     // Your logic here...
 }
 ```
 
 Instead of this:
 
 ```rust
#[tokio::main]
 async fn main() {
     // Your logic here...
 }
 ```
 
The second one produces very strange compiler messages because of the `#[tokio::main]` macro. However, the examples in this README use the second variant for brevity.

## FAQ
Q: Where I can ask questions?

A: [Issues](https://github.com/teloxide/teloxide/issues) is a good place for well-formed questions, for example, about the library design, enhancements, bug reports. But if you can't compile your bot due to compilation errors and need quick help, feel free to ask in [our official group](https://t.me/teloxide).

Q: Why Rust?

A: Most programming languages have their own implementations of Telegram bots frameworks, so why not Rust? We think Rust provides enough good ecosystem and the language itself to be suitable for writing bots.

Q: Can I use webhooks?

A: teloxide doesn't provide special API for working with webhooks due to their nature with lots of subtle settings. Instead, you setup your webhook by yourself, as shown in [`examples/ngrok_ping_pong_bot`](examples/ngrok_ping_pong_bot/src/main.rs) and [`examples/heroku_ping_pong_bot`](examples/heroku_ping_pong_bot/src/main.rs).

Associated links:
 - [Marvin's Marvellous Guide to All Things Webhook](https://core.telegram.org/bots/webhooks)
 - [Using self-signed certificates](https://core.telegram.org/bots/self-signed)

Q: Can I use different loggers?

A: Yes. The [`enable_logging!`](https://docs.rs/teloxide/latest/teloxide/macro.enable_logging.html) and [`enable_logging_with_filter!`](https://docs.rs/teloxide/latest/teloxide/macro.enable_logging_with_filter.html) macros are just convenient utilities, not necessary to use them. You can setup a different logger, for example, [fern](https://crates.io/crates/fern), as usual, e.g. teloxide has no specific requirements as it depends only on [log](https://crates.io/crates/log).

## Community bots
Feel free to push your own bot into our collection!

 - [Rust subreddit reader](https://github.com/steadylearner/Rust-Full-Stack/tree/master/commits/teloxide/subreddit_reader)
 - [with_webserver - An example of the teloxide + warp combination](https://github.com/steadylearner/Rust-Full-Stack/tree/master/commits/teloxide/with_webserver)
 - [vzmuinebot - Telegram bot for food menu navigate](https://github.com/ArtHome12/vzmuinebot)
 - [Tepe - A CLI to command a bot to send messages and files over Telegram](https://lib.rs/crates/tepe)

## Contributing
See [CONRIBUTING.md](https://github.com/teloxide/teloxide/blob/master/CONTRIBUTING.md).
