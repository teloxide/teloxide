<div align="center">
  <img src="ICON.png" width="250"/>
  <h1>teloxide</h1>
  
  <a href="https://docs.rs/teloxide/">
    <img src="https://img.shields.io/badge/docs.rs-v0.1.0-blue.svg">
  </a>
  <a href="https://github.com/teloxide/teloxide/actions">
    <img src="https://github.com/teloxide/teloxide/workflows/Continuous%20integration/badge.svg">
  </a>
  <a href="https://crates.io/crates/teloxide">
    <img src="https://img.shields.io/badge/crates.io-v0.1.0-orange.svg">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Features
 - **Type-safe.** teloxide leverages the Rust's type system with two serious implications: resistance to human mistakes and tight integration with IDEs. Write fast, avoid debugging.

## Getting started
 1. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 2. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
$ export TELOXIDE_TOKEN=MyAwesomeToken
```
 3. Be sure that you are up to date:
```bash
$ rustup update stable
```

 4. Execute `cargo new my_bot`, enter the directory and put these lines into your `Cargo.toml`:
```toml
[dependencies]
teloxide = "0.1.0"
log = "0.4.8"
tokio = "0.2.11"
pretty_env_logger = "0.4.0"
```

## The ping-pong bot
This bot just answers "pong" to each incoming message:

([Full](https://github.com/teloxide/teloxide/blob/dev/examples/ping_pong_bot/src/main.rs))
```rust
use teloxide::prelude::*;

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting the ping-pong bot!");

    let bot = Bot::from_env();

    Dispatcher::<RequestError>::new(bot)
        .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
            ctx.answer("pong").send().await?;
            Ok(())
        })
        .dispatch()
        .await;
}
```

## Commands
Commands are defined similar to how we define CLI using [structopt](https://docs.rs/structopt/0.3.9/structopt/):

([Full](https://github.com/teloxide/teloxide/blob/dev/examples/simple_commands_bot/src/main.rs))
```rust
// Imports are omitted...

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
            Ok(())
        }
    };

    let command = match Command::parse(text) {
        Some((command, _)) => command,
        None => {
            log::info!("Received a text message, but not a command.");
            Ok(())
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
    // Setup is omitted...
}

```

## Guess a number
Wanna see more? This is a bot, which starts a game on each incoming message. You must guess a number from 1 to 10 (inclusively):

([Full](https://github.com/teloxide/teloxide/blob/dev/examples/guess_a_number_bot/src/main.rs))
```rust
// Imports are omitted...

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveAttempt(u8),
}

async fn handle_message(
    ctx: DialogueHandlerCtx<Message, Dialogue>,
) -> Result<DialogueStage<Dialogue>, RequestError> {
    match ctx.dialogue {
        Dialogue::Start => {
            ctx.answer(
                "Let's play a game! Guess a number from 1 to 10 (inclusively).",
            )
            .send()
            .await?;
            next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
        }
        Dialogue::ReceiveAttempt(secret) => match ctx.update.text() {
            None => {
                ctx.answer("Oh, please, send me a text message!")
                    .send()
                    .await?;
                next(ctx.dialogue)
            }
            Some(text) => match text.parse::<u8>() {
                Ok(attempt) => match attempt {
                    x if !(1..=10).contains(&x) => {
                        ctx.answer(
                            "Oh, please, send me a number in the range [1; \
                             10]!",
                        )
                        .send()
                        .await?;
                        next(ctx.dialogue)
                    }
                    x if x == secret => {
                        ctx.answer("Congratulations! You won!").send().await?;
                        exit()
                    }
                    _ => {
                        ctx.answer("No.").send().await?;
                        next(ctx.dialogue)
                    }
                },
                Err(_) => {
                    ctx.answer("Oh, please, send me a number!").send().await?;
                    next(ctx.dialogue)
                }
            },
        },
    }
}

#[tokio::main]
async fn main() {
    teloxide::enable_logging!();
    log::info!("Starting guess_a_number_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .message_handler(&DialogueDispatcher::new(|ctx| async move {
            handle_message(ctx)
                .await
                .expect("Something wrong with the bot!")
        }))
        .dispatch()
        .await;
}
```

Our [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine), designating a user dialogue, cannot be in an invalid state. See [examples/dialogue_bot](https://github.com/teloxide/teloxide/blob/dev/examples/dialogue_bot/src/main.rs) to see a bit more complicated bot with dialogues.

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
 
The second one produces very strange compiler messages because of the `#[tokio::main]` macro. The examples in this README uses the first one for brevity.
