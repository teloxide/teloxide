//! A full-featured framework that empowers you to easily build [Telegram bots]
//! using the [`async`/`.await`] syntax in [Rust]. It handles all the difficult
//! stuff so you can focus only on your business logic.
//!
//! For a high-level overview, see [our GitHub repository](https://github.com/teloxide/teloxide).
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/dices_bot/src/main.rs))
//! ```no_run
//! use teloxide::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main_() {
//! teloxide::enable_logging!();
//! log::info!("Starting dices_bot...");
//!
//! let bot = Bot::from_env();
//!
//! teloxide::repl(bot, |message| async move {
//!     message.answer_dice().send().await?;
//!     ResponseResult::<()>::Ok(())
//! })
//! .await;
//! # }
//! ```
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/DICES_BOT.gif />
//!   </kbd>
//! </div>
//!
//! [Telegram bots]: https://telegram.org/blog/bot-revolution
//! [`async`/`.await`]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
//! [Rust]: https://www.rust-lang.org/

// https://github.com/teloxide/teloxide/raw/master/logo.svg doesn't work in html_logo_url, I don't know why.
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png"
)]
#![allow(clippy::match_bool)]
#![forbid(unsafe_code)]

pub use bot::{Bot, BotBuilder};
pub use dispatching::repls::{
    commands_repl, commands_repl_with_listener, dialogues_repl, dialogues_repl_with_listener, repl,
    repl_with_listener,
};
pub use errors::{ApiErrorKind, DownloadError, KnownApiErrorKind, RequestError};

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
