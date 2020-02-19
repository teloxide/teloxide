//! A full-featured framework that empowers you to easily build [Telegram bots]
//! using the [`async`/`.await`] syntax in [Rust]. It handles all the difficult
//! stuff so you can focus only on your business logic.
//!
//! # Features
//! - **Type-safe.** teloxide leverages the Rust's type system with two serious
//!   implications: resistance to human mistakes and tight integration with
//!   IDEs. Write fast, avoid debugging as much as possible.
//!
//! - **Flexible API.** teloxide gives you the power of [streams]: you can
//!   combine [all 30+ patterns] when working with updates from Telegram.
//!
//! - **Persistency.** By default, teloxide stores all user dialogues in RAM,
//!   but you can store them somewhere else (for example, in DB) just by
//!   implementing 2 functions.
//!
//! - **Convenient dialogues system.** Define a type-safe [finite automaton]
//! and transition functions to drive a user dialogue with ease (see [the
//! guess-a-number example](#guess-a-number) below).
//!
//! # Getting started
//!  1. Create a new bot using [@Botfather] to get a token in the format
//! `123456789:blablabla`.
//!  2. Initialise the `TELOXIDE_TOKEN` environmental
//! variable to your token:
//! ```text
//! # Unix
//! $ export TELOXIDE_TOKEN=<Your token here>
//!
//! # Windows
//! $ set TELOXITE_TOKEN=<Your token here>
//! ```
//!
//!  3. Be sure that you are up to date:
//! ```bash
//! $ rustup update stable
//! ```
//!
//!  4. Execute `cargo new my_bot`, enter the directory and put these lines into
//! your `Cargo.toml`:
//! ```text
//! [dependencies]
//! teloxide = "0.1.0"
//! log = "0.4.8"
//! tokio = "0.2.11"
//! pretty_env_logger = "0.4.0"
//! ```
//!
//! # The ping-pong bot
//! This bot has a single message handler, which answers "pong" to each incoming
//! message:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/src/main.rs))
//! ```no_run
//! use teloxide::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     teloxide::enable_logging!();
//!     log::info!("Starting ping_pong_bot!");
//!
//!     let bot = Bot::from_env();
//!
//!     Dispatcher::new(bot)
//!         .messages_handler(|rx: DispatcherHandlerRx<Message>| {
//!             rx.for_each(|message| async move {
//!                 message.answer("pong").send().await.log_on_error().await;
//!             })
//!         })
//!         .dispatch()
//!         .await;
//! }
//! ```
//!
//! <details>
//! <summary>Click here to run it!</summary>
//!
//! ```text
//! git clone https://github.com/teloxide/teloxide.git
//! cd teloxide/examples/ping_pong_bot
//! TELOXIDE_TOKEN=<Your token here> cargo run
//! ```
//!
//! </details>
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/PING_PONG_BOT.png width="600" />
//!   </kbd>
//! </div>
//!
//! # Commands
//! Commands are defined similar to how we define CLI using [structopt]. This
//! bot says "I am a cat! Meow!" on `/meow`, generates a random number within
//! [0; 1) on `/generate`, and shows the usage guide on `/help`:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/simple_commands_bot/src/main.rs))
//! ```no_run
//! // Imports are omitted...
//! # use teloxide::{prelude::*, utils::command::BotCommand};
//! # use rand::{thread_rng, Rng};
//!
//! #[derive(BotCommand)]
//! #[command(
//!     rename = "lowercase",
//!     description = "These commands are supported:"
//! )]
//! enum Command {
//!     #[command(description = "display this text.")]
//!     Help,
//!     #[command(description = "be a cat.")]
//!     Meow,
//!     #[command(description = "generate a random number within [0; 1).")]
//!     Generate,
//! }
//!
//! fn generate() -> String {
//!     thread_rng().gen_range(0.0, 1.0).to_string()
//! }
//!
//! async fn answer(
//!     cx: DispatcherHandlerCx<Message>,
//!     command: Command,
//! ) -> ResponseResult<()> {
//!     match command {
//!         Command::Help => cx.answer(Command::descriptions()).send().await?,
//!         Command::Generate => cx.answer(generate()).send().await?,
//!         Command::Meow => cx.answer("I am a cat! Meow!").send().await?,
//!     };
//!
//!     Ok(())
//! }
//!
//! async fn handle_command(rx: DispatcherHandlerRx<Message>) {
//!    // Only iterate through text messages:
//!     rx.text_messages()
//!         // Only iterate through commands in a proper format:
//!         .commands::<Command>()
//!         // Execute all incoming commands concurrently:
//!         .for_each_concurrent(None, |(cx, command, _)| async move {
//!             answer(cx, command).await.log_on_error().await;
//!         })
//!         .await;
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup is omitted...
//! # teloxide::enable_logging!();
//! # log::info!("Starting simple_commands_bot!");
//! # let bot = Bot::from_env();
//! # Dispatcher::new(bot).messages_handler(handle_command).dispatch().await;
//! }
//! ```
//!
//! <details>
//! <summary>Click here to run it!</summary>
//!
//! ```text
//! git clone https://github.com/teloxide/teloxide.git
//! cd teloxide/examples/simple_commands_bot
//! TELOXIDE_TOKEN=<Your token here> cargo run
//! ```
//!
//! </details>
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/SIMPLE_COMMANDS_BOT.png width="500"/>
//!   </kbd>
//!   <br/><br/>
//! </div>
//!
//!
//! See? The dispatcher gives us a stream of messages, so we can handle it as we
//! want! Here we use our `.text_messages()`, `.commands()`, and
//! [`.for_each_concurrent()`], but others are also available:
//! - [`.flatten()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.flatten)
//! - [`.left_stream()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.left_stream)
//! - [`.scan()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.scan)
//! - [`.skip_while()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.skip_while)
//! - [`.zip()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.zip)
//! - [`.select_next_some()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.select_next_some)
//! - [`.fold()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.fold)
//! - [`.inspect()`](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.inspect)
//! - ... And lots of [others](https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html)!
//!
//! # Guess a number
//! Wanna see more? This is a bot, which starts a game on each incoming message.
//! You must guess a number from 1 to 10 (inclusively):
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/guess_a_number_bot/src/main.rs))
//! ```no_run
//! // Setup is omitted...
//! # #[macro_use]
//! # extern crate smart_default;
//! # use teloxide::prelude::*;
//! # use rand::{thread_rng, Rng};
//!
//! #[derive(SmartDefault)]
//! enum Dialogue {
//!     #[default]
//!     Start,
//!     ReceiveAttempt(u8),
//! }
//!
//! async fn handle_message(
//!     cx: DialogueDispatcherHandlerCx<Message, Dialogue>,
//! ) -> ResponseResult<DialogueStage<Dialogue>> {
//!     match cx.dialogue {
//!         Dialogue::Start => {
//!             cx.answer(
//!                 "Let's play a game! Guess a number from 1 to 10 \
//!                      (inclusively).",
//!             )
//!             .send()
//!             .await?;
//!             next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
//!         }
//!         Dialogue::ReceiveAttempt(secret) => match cx.update.text() {
//!             None => {
//!                 cx.answer("Oh, please, send me a text message!")
//!                     .send()
//!                     .await?;
//!                 next(cx.dialogue)
//!             }
//!             Some(text) => match text.parse::<u8>() {
//!                 Ok(attempt) => match attempt {
//!                     x if !(1..=10).contains(&x) => {
//!                         cx.answer(
//!                             "Oh, please, send me a number in the range \
//!                                  [1; 10]!",
//!                         )
//!                         .send()
//!                         .await?;
//!                         next(cx.dialogue)
//!                     }
//!                     x if x == secret => {
//!                         cx.answer("Congratulations! You won!")
//!                             .send()
//!                             .await?;
//!                         exit()
//!                     }
//!                     _ => {
//!                         cx.answer("No.").send().await?;
//!                         next(cx.dialogue)
//!                     }
//!                 },
//!                 Err(_) => {
//!                     cx.answer(
//!                         "Oh, please, send me a number in the range [1; \
//!                              10]!",
//!                     )
//!                     .send()
//!                     .await?;
//!                     next(cx.dialogue)
//!                 }
//!             },
//!         },
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup is omitted...
//! # teloxide::enable_logging!();
//! # log::info!("Starting guess_a_number_bot!");
//! # let bot = Bot::from_env();
//! # Dispatcher::new(bot)
//! # .messages_handler(DialogueDispatcher::new(|cx| async move {
//! # handle_message(cx).await.expect("Something wrong with the bot!")
//! # }))
//! # .dispatch()
//! # .await;
//! }
//! ```
//!
//! <details>
//! <summary>Click here to run it!</summary>
//!
//! ```text
//! git clone https://github.com/teloxide/teloxide.git
//! cd teloxide/examples/guess_a_number_bot
//! TELOXIDE_TOKEN=<Your token here> cargo run
//! ```
//!
//! </details>
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/GUESS_A_NUMBER_BOT.png width="600" />
//!   </kbd>
//!   <br/><br/>
//! </div>
//!
//! Our [finite automaton], designating a user dialogue, cannot be in an invalid
//! state, and this is why it is called "type-safe". We could use `enum` +
//! `Option`s instead, but it will lead is to lots of unpleasure `.unwrap()`s.
//!
//! Remember that a classical [finite automaton] is defined by its initial
//! state, a list of its possible states and a transition function? We can think
//! that `Dialogue` is a finite automaton with a context type at each state
//! (`Dialogue::Start` has `()`, `Dialogue::ReceiveAttempt` has `u8`).
//!
//! See [examples/dialogue_bot] to see a bit more complicated bot with
//! dialogues.
//!
//! # [More examples!](https://github.com/teloxide/teloxide/tree/master/examples)
//!
//! # Recommendations
//!
//!  - Use this pattern:
//!
//!  ```rust
//!  #[tokio::main]
//!  async fn main() {
//!      run().await;
//!  }
//!
//!  async fn run() {
//!      // Your logic here...
//!  }
//!  ```
//!
//!  Instead of this:
//!
//!  ```rust
//! #[tokio::main]
//!  async fn main() {
//!      // Your logic here...
//!  }
//!  ```
//!
//!  The second one produces very strange compiler messages because of the
//! `#[tokio::main]` macro. However, the examples above use the second one for
//! brevity.
//!
//! [Telegram bots]: https://telegram.org/blog/bot-revolution
//! [`async`/`.await`]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
//! [Rust]: https://www.rust-lang.org/
//! [finite automaton]: https://en.wikipedia.org/wiki/Finite-state_machine
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/blob/master/examples/dialogue_bot/src/main.rs
//! [structopt]: https://docs.rs/structopt/0.3.9/structopt/
//! [@Botfather]: https://t.me/botfather
//! [streams]: https://docs.rs/futures/0.3.4/futures/stream/index.html
//! [all 30+ patterns]: https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html
//! [`.for_each_concurrent()`]: https://docs.rs/futures/0.3.4/futures/stream/trait.StreamExt.html#method.for_each_concurrent
//! [See more examples]: https://github.com/teloxide/teloxide/tree/master/examples
//! [category theory]: https://en.wikipedia.org/wiki/Category_theory
//! [coproduct]: https://en.wikipedia.org/wiki/Coproduct

#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/master/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png"
)]
#![allow(clippy::match_bool)]

pub use bot::Bot;
pub use errors::{ApiErrorKind, DownloadError, RequestError};

mod errors;
mod net;

mod bot;
pub mod dispatching;
pub mod error_handlers;
mod logging;
pub mod prelude;
pub mod requests;
pub mod types;
pub mod utils;

extern crate teloxide_macros;
