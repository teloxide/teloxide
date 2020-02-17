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
//! ## Getting started
//!  1. Create a new bot using [@Botfather] to get a token in the format
//! `123456789:blablabla`.  2. Initialise the `TELOXIDE_TOKEN` environmental
//! variable to your token:
//! ```bash
//! # Unix
//! $ export TELOXIDE_TOKEN=MyAwesomeToken
//!
//! # Windows
//! $ set TELOXITE_TOKEN=MyAwesomeToken
//! ```
//!  3. Be sure that you are up to date:
//! ```bash
//! $ rustup update stable
//! ```
//!
//!  4. Execute `cargo new my_bot`, enter the directory and put these lines into
//! your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! teloxide = "0.1.0"
//! log = "0.4.8"
//! tokio = "0.2.11"
//! pretty_env_logger = "0.4.0"
//! ```
//!
//! <div align="center">
//!   <img src=https://github.com/teloxide/teloxide/raw/master/media/GUESS_A_NUMBER_BOT.png width="400" />
//! </div>
//!
//! Our [finite automaton], designating a user dialogue, cannot be in an invalid
//! state. See [examples/dialogue_bot] to see a bit more complicated bot with
//! dialogues.
//!
//! [See more examples](https://github.com/teloxide/teloxide/tree/master/examples).
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
mod logging;
pub mod prelude;
pub mod requests;
pub mod types;
pub mod utils;

extern crate teloxide_macros;
