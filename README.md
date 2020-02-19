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
  <a href="https://t.me/teloxide">
    <img src="https://img.shields.io/badge/official%20chat-t.me%2Fteloxide-blueviolet">
  </a>
  
  A full-featured framework that empowers you to easily build [Telegram bots](https://telegram.org/blog/bot-revolution) using the [`async`/`.await`](https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html) syntax in [Rust](https://www.rust-lang.org/). It handles all the difficult stuff so you can focus only on your business logic.
</div>

## Table of contents
 - [Features](https://github.com/teloxide/teloxide#features)
 - [Getting started](https://github.com/teloxide/teloxide#getting-started)
 - [Examples](https://github.com/teloxide/teloxide#examples)
   - [The ping-pong bot](https://github.com/teloxide/teloxide#the-ping-pong-bot)
   - [Commands](https://github.com/teloxide/teloxide#commands)
   - [Guess a number](https://github.com/teloxide/teloxide#guess-a-number)
 - [More examples!](https://github.com/teloxide/teloxide#more-examples)
 - [Recommendations](https://github.com/teloxide/teloxide#recommendations)
 - [FAQ](https://github.com/teloxide/teloxide#faq)
   - [Where I can ask questions?](https://github.com/teloxide/teloxide#where-i-can-ask-questions)
   - [Why Rust?](https://github.com/teloxide/teloxide#why-rust)
 - [Community bots](https://github.com/teloxide/teloxide#community-bots)
 - [Contributing](https://github.com/teloxide/teloxide#contributing)

## Features
 - **Type-safe.** teloxide leverages the Rust's type system with two serious implications: resistance to human mistakes and tight integration with IDEs. Write fast, avoid debugging as much as possible.

 - **Flexible API.** teloxide gives you the power of [streams](https://docs.rs/futures/0.3.4/futures/stream/index.html): you can combine [all 30+ patterns](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html) when working with updates from Telegram.

 - **Persistency.** By default, teloxide stores all user dialogues in RAM, but you can store them somewhere else (for example, in DB) just by implementing 2 functions.
  
 - **Convenient dialogues system.** Define a type-safe [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine)
 and transition functions to drive a user dialogue with ease (see [the guess-a-number example](#guess-a-number) below).
 
## Getting started
 1. Create a new bot using [@Botfather](https://t.me/botfather) to get a token in the format `123456789:blablabla`.
 2. Initialise the `TELOXIDE_TOKEN` environmental variable to your token:
```bash
# Unix
$ export TELOXIDE_TOKEN=<Your token here>

# Windows
$ set TELOXITE_TOKEN=<Your token here>
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

<details>
  <summary>Click here to run it!</summary>

```bash
git clone https://github.com/teloxide/teloxide.git
cd teloxide/examples/ping_pong_bot
TELOXIDE_TOKEN=<Your token here> cargo run
```

</details>

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/PING_PONG_BOT.png width="600" />
  </kbd>
</div>

## Commands
Commands are defined similar to how we define CLI using [structopt](https://docs.rs/structopt/0.3.9/structopt/). This bot says "I am a cat! Meow!" on `/meow`, generates a random number within [0; 1) on `/generate`, and shows the usage guide on `/help`:

([Full](https://github.com/teloxide/teloxide/blob/master/examples/simple_commands_bot/src/main.rs))
```rust
// Imports are omitted...

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
    // Only iterate through text messages:
    rx.text_messages()
        // Only iterate through commands in a proper format:
        .commands::<Command>()
        // Execute all incoming commands concurrently:
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    // Setup is omitted...
}
```

<details>
  <summary>Click here to run it!</summary>

```bash
git clone https://github.com/teloxide/teloxide.git
cd teloxide/examples/simple_commands_bot
TELOXIDE_TOKEN=<Your token here> cargo run
```

</details>

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/SIMPLE_COMMANDS_BOT.png width="500"/>
  </kbd>
  <br/><br/>
</div>


See? The dispatcher gives us a stream of messages, so we can handle it as we want! Here we use our `.text_messages()`, `.commands()`, and [`.for_each_concurrent()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.for_each_concurrent), but others are also available:
 - [`.flatten()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.flatten)
 - [`.left_stream()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.left_stream)
 - [`.scan()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.scan)
 - [`.skip_while()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.skip_while)
 - [`.zip()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.zip)
 - [`.select_next_some()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.select_next_some)
 - [`.fold()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.fold)
 - [`.inspect()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.inspect)
 - ... And lots of [others](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html)!

## Guess a number
Wanna see more? This is a bot, which starts a game on each incoming message. You must guess a number from 1 to 10 (inclusively):

([Full](https://github.com/teloxide/teloxide/blob/master/examples/guess_a_number_bot/src/main.rs))
```rust
// Setup is omitted...

#[derive(SmartDefault)]
enum Dialogue {
    #[default]
    Start,
    ReceiveAttempt(u8),
}

async fn handle_message(
    cx: DialogueDispatcherHandlerCx<Message, Dialogue>,
) -> ResponseResult<DialogueStage<Dialogue>> {
    match cx.dialogue {
        Dialogue::Start => {
            cx.answer(
                "Let's play a game! Guess a number from 1 to 10 (inclusively).",
            )
            .send()
            .await?;
            next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
        }
        Dialogue::ReceiveAttempt(secret) => match cx.update.text() {
            None => {
                cx.answer("Oh, please, send me a text message!").send().await?;
                next(cx.dialogue)
            }
            Some(text) => match text.parse::<u8>() {
                Ok(attempt) => match attempt {
                    x if !(1..=10).contains(&x) => {
                        cx.answer(
                            "Oh, please, send me a number in the range [1; \
                             10]!",
                        )
                        .send()
                        .await?;
                        next(cx.dialogue)
                    }
                    x if x == secret => {
                        cx.answer("Congratulations! You won!").send().await?;
                        exit()
                    }
                    _ => {
                        cx.answer("No.").send().await?;
                        next(cx.dialogue)
                    }
                },
                Err(_) => {
                    cx.answer(
                        "Oh, please, send me a number in the range [1; 10]!",
                    )
                    .send()
                    .await?;
                    next(cx.dialogue)
                }
            },
        },
    }
}

#[tokio::main]
async fn main() {
    // Setup is omitted...
}
```

<details>
  <summary>Click here to run it!</summary>

```bash
git clone https://github.com/teloxide/teloxide.git
cd teloxide/examples/guess_a_number_bot
TELOXIDE_TOKEN=<Your token here> cargo run
```

</details>

<div align="center">
  <kbd>
    <img src=https://github.com/teloxide/teloxide/raw/master/media/GUESS_A_NUMBER_BOT.png width="600" />
  </kbd>
  <br/><br/>
</div>

Our [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine), designating a user dialogue, cannot be in an invalid state, and this is why it is called "type-safe". We could use `enum` + `Option`s instead, but it will lead is to lots of unpleasure `.unwrap()`s.

Remember that a classical [finite automaton](https://en.wikipedia.org/wiki/Finite-state_machine) is defined by its initial state, a list of its possible states and a transition function? We can think that `Dialogue` is a finite automaton with a context type at each state (`Dialogue::Start` has `()`, `Dialogue::ReceiveAttempt` has `u8`).

See [examples/dialogue_bot](https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/main.rs) to see a bit more complicated bot with dialogues.

## [More examples!](https://github.com/teloxide/teloxide/tree/master/examples)

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
### Where I can ask questions?
[Issues](https://github.com/teloxide/teloxide/issues) is a good place for well-formed questions, for example, about the library design, enhancements, bug reports. But if you can't compile your bot due to compilation errors and need a quick help, feel free to ask in our official group: https://t.me/teloxide.

### Why Rust?
Most programming languages have their own implementations of Telegram bots frameworks, so why not Rust? We think Rust provides enough good ecosystem and the language itself to be suitable for writing bots.

## Community bots
Feel free to push your own bot into our collection: https://github.com/teloxide/community-bots. Later you will be able to play with them right in our official chat: https://t.me/teloxide.

## Contributing
See [CONRIBUTING.md](https://github.com/teloxide/teloxide/blob/master/CONTRIBUTING.md).
