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
//! let bot = Bot::from_env().auto_send();
//!
//! teloxide::repl(bot, |message| async move {
//!     message.answer_dice().await?;
//!     respond(())
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

// This hack is used to cancel formatting for a Markdown table. See [1], [2], and [3].
//
// [1]: https://github.com/rust-lang/rustfmt/issues/4210
// [2]: https://github.com/rust-lang/rustfmt/issues/4787
// [3]: https://github.com/rust-lang/rust/issues/82768#issuecomment-803935643
#![cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("features.txt")))]
// https://github.com/teloxide/teloxide/raw/master/logo.svg doesn't work in html_logo_url, I don't know why.
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/master/ICON.png"
)]
#![allow(clippy::match_bool)]
#![forbid(unsafe_code)]
// we pass "--cfg docsrs" when building docs to add `This is supported on
// feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs -Znormalize-docs" cargo +nightly doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg))]

pub use dispatching::repls::{
    commands_repl, commands_repl_with_listener, dialogues_repl, dialogues_repl_with_listener, repl,
    repl_with_listener,
};

mod logging;

pub mod dispatching;
pub mod error_handlers;
pub mod prelude;
pub mod utils;

#[doc(inline)]
pub use teloxide_core::*;

#[cfg(feature = "macros")]
#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
pub use teloxide_macros as macros;

#[cfg_attr(all(docsrs, feature = "nightly"), doc(cfg(feature = "macros")))]
#[cfg(feature = "macros")]
pub use teloxide_macros::teloxide;

#[cfg(all(feature = "nightly", doctest))]
#[cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("../README.md")))]
enum ReadmeDocTests {}

use teloxide_core::requests::ResponseResult;

/// A shortcut for `ResponseResult::Ok(val)`.
pub fn respond<T>(val: T) -> ResponseResult<T> {
    ResponseResult::Ok(val)
}
