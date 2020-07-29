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
    <img src="https://img.shields.io/badge/API coverage-Up to 0.4.7 (inclusively)-green.svg">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Table of contents
 - [Highlights](https://github.com/teloxide/teloxide#highlights)
 - [Setting up your environment](https://github.com/teloxide/teloxide#setting-up-your-environment)
 - [API overview](https://github.com/teloxide/teloxide#api-overview)
   - [The dices bot](https://github.com/teloxide/teloxide#the-dices-bot)
   - [Commands](https://github.com/teloxide/teloxide#commands)
   - [Dialogues management](https://github.com/teloxide/teloxide#dialogues-management)
 - [Recommendations](https://github.com/teloxide/teloxide#recommendations)
 - [Cargo features](https://github.com/teloxide/teloxide#cargo-features)
 - [FAQ](https://github.com/teloxide/teloxide#faq)
 - [Community bots](https://github.com/teloxide/teloxide#community-bots)
 - [Contributing](https://github.com/teloxide/teloxide#contributing)

## Highlights

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
teloxide-macros = "0.3.2"

log = "0.4.8"
pretty_env_logger = "0.4.0"

tokio = "0.2.11"
futures = "0.3.5"
```

## API overview

### The dices bot
This bot throws a dice on each incoming message:

([Full](https://github.com/teloxide/teloxide/blob/master/examples/dices_bot/src/main.rs))
```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dices_bot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {
                message.send_dice().send().await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}

```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/DICES_BOT.gif />
  </kbd>
</div>

### Commands
Commands are strongly typed and defined declaratively, similar to how we define CLI using [structopt] and JSON structures in [serde-json]. The following bot accepts these commands:

 - `/username <your username>`
 - `/usernameandage <your username> <your age>`
 - `/help`

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
    #[command(description = "handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn answer(cx: UpdateWithCx<Message>, command: Command) -> ResponseResult<()> {
    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Username(username) => {
            cx.answer_str(format!("Your username is @{}.", username)).await?
        }
        Command::UsernameAndAge { username, age } => {
            cx.answer_str(format!("Your username is @{} and age is {}.", username, age)).await?
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

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/SIMPLE_COMMANDS_BOT.gif />
  </kbd>
</div>

### Dialogues management
A dialogue is described by an enumeration, where each variant is one of possible dialogue's states. There are also _subtransition functions_, which turn a dialogue from one state to another, thereby forming a [FSM].

[FSM]: https://en.wikipedia.org/wiki/Finite-state_machine

Below is a bot, which asks you three questions and then sends the answers back to you. First, let's start with an enumeration (a collection of our dialogue's states):

([dialogue_bot/src/dialogue/mod.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/dialogue/mod.rs))
```rust
// Imports are omitted...

#[derive(Transition, From)]
pub enum Dialogue {
    Start(StartState),
    ReceiveFullName(ReceiveFullNameState),
    ReceiveAge(ReceiveAgeState),
    ReceiveLocation(ReceiveLocationState),
}

impl Default for Dialogue {
    fn default() -> Self {
        Self::Start(StartState)
    }
}
```

When a user sends a message to our bot, and such a dialogue does not yet exist, `Dialogue::default()` is invoked, which is `Dialogue::Start`. Every time a message is received, an associated dialogue is extracted, and then passed to a corresponding subtransition function:

<details>
  <summary>Dialogue::Start</summary>

([dialogue_bot/src/dialogue/states/start.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/dialogue/states/start.rs))
```rust
// Imports are omitted...

pub struct StartState;

#[teloxide(subtransition)]
async fn start(_state: StartState, cx: TransitionIn, _ans: String) -> TransitionOut<Dialogue> {
    cx.answer_str("Let's start! What's your full name?").await?;
    next(ReceiveFullNameState)
}
```

</details>

<details>
  <summary>Dialogue::ReceiveFullName</summary>

([dialogue_bot/src/dialogue/states/receive_full_name.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/dialogue/states/receive_full_name.rs))
```rust
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveFullNameState;

#[teloxide(subtransition)]
async fn receive_full_name(
    state: ReceiveFullNameState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str("How old are you?").await?;
    next(ReceiveAgeState::up(state, ans))
}
```

</details>

<details>
  <summary>Dialogue::ReceiveAge</summary>

([dialogue_bot/src/dialogue/states/receive_age.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/dialogue/states/receive_age.rs))
```rust
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveAgeState {
    pub full_name: String,
}

#[teloxide(subtransition)]
async fn receive_age_state(
    state: ReceiveAgeState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    match ans.parse::<u8>() {
        Ok(ans) => {
            cx.answer_str("What's your location?").await?;
            next(ReceiveLocationState::up(state, ans))
        }
        _ => {
            cx.answer_str("Send me a number.").await?;
            next(state)
        }
    }
}
```

</details>

<details>
    <summary>Dialogue::ReceiveLocation</summary>

([dialogue_bot/src/dialogue/states/receive_location.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/dialogue/states/receive_location.rs))
```rust
// Imports are omitted...

#[derive(Generic)]
pub struct ReceiveLocationState {
    pub full_name: String,
    pub age: u8,
}

#[teloxide(subtransition)]
async fn receive_location(
    state: ReceiveLocationState,
    cx: TransitionIn,
    ans: String,
) -> TransitionOut<Dialogue> {
    cx.answer_str(format!("Full name: {}\nAge: {}\nLocation: {}", state.full_name, state.age, ans))
        .await?;
    exit()
}
```

</details>

All these subtransitions accept a corresponding state (one of the many variants of `Dialogue`), a context, and a textual message. They return `TransitionOut<Dialogue>`, e.g. a mapping from `<your state type>` to `Dialogue`.

Finally, the `main` function looks like this:

([dialogue_bot/src/main.rs](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/main.rs))
```rust
// Imports are omitted...

type In = DialogueWithCx<Message, Dialogue, Infallible>;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting dialogue_bot...");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(DialogueDispatcher::new(
            |DialogueWithCx { cx, dialogue }: In| async move {
                let dialogue = dialogue.expect("std::convert::Infallible");
                handle_message(cx, dialogue).await.expect("Something wrong with the bot!")
            },
        ))
        .dispatch()
        .await;
}

async fn handle_message(cx: UpdateWithCx<Message>, dialogue: Dialogue) -> TransitionOut<Dialogue> {
    match cx.update.text_owned() {
        None => {
            cx.answer_str("Send me a text message.").await?;
            next(dialogue)
        }
        Some(ans) => dialogue.react(cx, ans).await,
    }
}

```

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/DIALOGUE_BOT.gif />
  </kbd>
</div>

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

## Cargo features

 - `redis-storage` -- enables the [Redis] support.
 - `cbor-serializer` -- enables the [CBOR] serializer for dialogues.
 - `bincode-serializer` -- enables the [Bincode] serializer for dialogues.
 - `frunk` -- enables [`teloxide::utils::UpState`], which allows mapping from a structure of `field1, ..., fieldN` to a structure of `field1, ..., fieldN, fieldN+1`.

[CBOR]: https://en.wikipedia.org/wiki/CBOR
[Bincode]: https://github.com/servo/bincode
[`teloxide::utils::UpState`]: https://docs.rs/teloxide/latest/teloxide/utils/trait.UpState.html

## FAQ
Q: Where I can ask questions?

A: [Issues](https://github.com/teloxide/teloxide/issues) is a good place for well-formed questions, for example, about:

 - the library design;
 - enhancements;
 - bug reports;
 - ...

If you can't compile your bot due to compilation errors and need quick help, feel free to ask in [our official Telegram group](https://t.me/teloxide).

Q: Do you support the Telegram API for clients?

A: No, only the bots API.

Q: Why Rust?

A: Most programming languages have their own implementations of Telegram bots frameworks, so why not Rust? We think Rust provides enough good ecosystem and the language itself to be suitable for writing bots.

Q: Can I use webhooks?

A: teloxide doesn't provide special API for working with webhooks due to their nature with lots of subtle settings. Instead, you setup your webhook by yourself, as shown in [`examples/ngrok_ping_pong_bot`](examples/ngrok_ping_pong_bot/src/main.rs) and [`examples/heroku_ping_pong_bot`](examples/heroku_ping_pong_bot/src/main.rs).

Associated links:
 - [Marvin's Marvellous Guide to All Things Webhook](https://core.telegram.org/bots/webhooks)
 - [Using self-signed certificates](https://core.telegram.org/bots/self-signed)

Q: Can I use different loggers?

A: Yes. You can setup any logger, for example, [fern], e.g. teloxide has no specific requirements as it depends only on [log]. Remember that [`enable_logging!`] and [`enable_logging_with_filter!`] are just **optional** utilities.

[fern]: https://crates.io/crates/fern
[log]: https://crates.io/crates/log
[`enable_logging!`]: https://docs.rs/teloxide/latest/teloxide/macro.enable_logging.html
[`enable_logging_with_filter!`]: https://docs.rs/teloxide/latest/teloxide/macro.enable_logging_with_filter.html

## Community bots
Feel free to push your own bot into our collection!

 - [_Rust subreddit reader_](https://github.com/steadylearner/Rust-Full-Stack/tree/master/commits/teloxide/subreddit_reader)
 - [_vzmuinebot -- Telegram bot for food menu navigate_](https://github.com/ArtHome12/vzmuinebot)
 - [_Tepe -- A CLI to command a bot to send messages and files over Telegram_](https://lib.rs/crates/tepe)

## Contributing
See [CONRIBUTING.md](https://github.com/teloxide/teloxide/blob/master/CONTRIBUTING.md).
