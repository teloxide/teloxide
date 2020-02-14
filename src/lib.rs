//! A full-featured framework that empowers you to easily build [Telegram bots]
//! using the [`async`/`.await`] syntax in [Rust]. It handles all the difficult
//! stuff so you can focus only on your business logic.
//!
//! ## Features
//!  - **Type-safe.** teloxide leverages the Rust's type system with two serious
//!    implications: resistance to human mistakes and tight integration with
//!    IDEs. Write fast, avoid debugging as possible.
//!
//!  - **Persistency.** By default, teloxide stores all user dialogues in RAM,
//!    but you can store them somewhere else (for example, in DB) just by
//!    implementing 2 functions.
//!
//!  - **Convenient dialogues system.** Define a type-safe [finite automaton]
//!    and transition functions to drive a user dialogue with ease (see the
//! examples below).
//!
//!  - **Convenient API.** Automatic conversions are used to avoid boilerplate.
//!    For example, functions accept `Into<String>`, rather than `&str` or
//!    `String`, so you can call them without `.to_string()`/`.as_str()`/etc.
//!
//! ## The ping-pong bot
//! This bot has a single message handler, which answers "pong" to each incoming
//!  message:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/dev/examples/ping_pong_bot/src/main.rs))
//! ```rust
//! use teloxide::prelude::*;
//!
//! #[tokio::main]
//! async fn main() {
//!     teloxide::enable_logging!();
//!     log::info!("Starting the ping-pong bot!");
//!
//!     let bot = Bot::from_env();
//!
//!     Dispatcher::<RequestError>::new(bot)
//!         .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
//!             ctx.answer("pong").send().await?;
//!             Ok(())
//!         })
//!         .dispatch()
//!         .await;
//! }
//! ```
//!
//! ## Commands
//! Commands are defined similar to how we define CLI using [structopt]. This
//! bot says "I am a cat! Meow!" on `/meow`, generates a random number within
//! [0; 1) on `/generate`, and shows the usage guide on `/help`:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/dev/examples/simple_commands_bot/src/main.rs))
//! ```rust
//! // Imports are omitted...
//!
//! #[derive(BotCommand)]
//! #[command(
//!     rename = "lowercase",
//!     description = "These commands are
//! #[command(rename supported:"
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
//! async fn handle_command(
//!     ctx: DispatcherHandlerCtx<Message>,
//! ) -> Result<(), RequestError> {
//!     let text = match ctx.update.text() {
//!         Some(text) => text,
//!         None => {
//!             log::info!("Received a message, but not text.");
//!             return Ok(());
//!         }
//!     };
//!
//!     let command = match Command::parse(text) {
//!         Some((command, _)) => command,
//!         None => {
//!             log::info!("Received a text message, but not a command.");
//!             return Ok(());
//!         }
//!     };
//!
//!     match command {
//!         Command::Help => ctx.answer(Command::descriptions()).send().await?,
//!         Command::Generate => {
//!             ctx.answer(thread_rng().gen_range(0.0, 1.0).to_string())
//!                 .send()
//!                 .await?
//!         }
//!         Command::Meow => ctx.answer("I am a cat! Meow!").send().await?,
//!     };
//!
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup is omitted...
//! }
//! ```
//!
//! ## Guess a number
//! Wanna see more? This is a bot, which starts a game on each incoming message.
//!  You must guess a number from 1 to 10 (inclusively):
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/dev/examples/guess_a_number_bot/src/main.rs))
//! ```rust
//! // Imports are omitted...
//!
//! #[derive(SmartDefault)]
//! enum Dialogue {
//!     #[default]
//!     Start,
//!     ReceiveAttempt(u8),
//! }
//! async fn handle_message(
//!     ctx: DialogueHandlerCtx<Message, Dialogue>,
//! ) -> Result<DialogueStage<Dialogue>, RequestError> {
//!     match ctx.dialogue {
//!         Dialogue::Start => {
//!             ctx.answer(
//!                 "Let's play a game! Guess a number from 1 to 10
//!  (inclusively).",
//!             )
//!             .send()
//!             .await?;
//!             next(Dialogue::ReceiveAttempt(thread_rng().gen_range(1, 11)))
//!         }
//!         Dialogue::ReceiveAttempt(secret) => match ctx.update.text() {
//!             None => {
//!                 ctx.answer("Oh, please, send me a text message!")
//!                     .send()
//!                     .await?;
//!                 next(ctx.dialogue)
//!             }
//!             Some(text) => match text.parse::<u8>() {
//!                 Ok(attempt) => match attempt {
//!                     x if !(1..=10).contains(&x) => {
//!                         ctx.answer(
//!                             "Oh, please, send me a number in the range \
//!                                  [1; 10]!",
//!                         )
//!                         .send()
//!                         .await?;
//!                         next(ctx.dialogue)
//!                     }
//!                     x if x == secret => {
//!                         ctx.answer("Congratulations! You won!")
//!                             .send()
//!                             .await?;
//!                         exit()
//!                     }
//!                     _ => {
//!                         ctx.answer("No.").send().await?;
//!                         next(ctx.dialogue)
//!                     }
//!                 },
//!                 Err(_) => {
//!                     ctx.answer(
//!                         "Oh, please, send me a number in the range [1; \
//!                              10]!",
//!                     )
//!                     .send()
//!                     .await?;
//!                     next(ctx.dialogue)
//!                 }
//!             },
//!         },
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup is omitted...
//!
//!     Dispatcher::new(bot)
//!         .message_handler(&DialogueDispatcher::new(|ctx| async move {
//!             handle_message(ctx)
//!                 .await
//!                 .expect("Something wrong with the bot!")
//!         }))
//!         .dispatch()
//!         .await;
//! }
//! ```
//!
//! Our [finite automaton], designating a user dialogue, cannot be in an invalid
//! state. See [examples/dialogue_bot] to see a bit more complicated bot with
//! dialogues.
//!
//! [See more examples](https://github.com/teloxide/teloxide/tree/dev/examples).
//!
//! ## Recommendations
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
//! `#[tokio::main]` macro. The examples above use the first one for brevity.
//!
//! [Telegram bots]: https://telegram.org/blog/bot-revolution
//! [`async`/`.await`]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
//! [Rust]: https://www.rust-lang.org/
//! [finite automaton]: https://en.wikipedia.org/wiki/Finite-state_machine
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/blob/dev/examples/dialogue_bot/src/main.rs
//! [structopt]: https://docs.rs/structopt/0.3.9/structopt/

#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/dev/logo.svg",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/dev/ICON.png"
)]
#![allow(clippy::match_bool)]

pub use bot::Bot;
pub use errors::{ApiErrorKind, DownloadError, RequestError};

mod errors;
mod net;

mod bot;
pub mod dispatching;
mod logging;
pub mod prelude;
pub mod requests;
pub mod types;
pub mod utils;

extern crate teloxide_macros;
