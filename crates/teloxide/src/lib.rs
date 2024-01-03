//! A full-featured framework that empowers you to easily build [Telegram bots]
//! using [Rust]. It handles all the difficult stuff so you can focus only on
//! your business logic.
//!
//! For a high-level overview, see [our GitHub repository](https://github.com/teloxide/teloxide).
//!
//! [[`examples/throw_dice.rs`](https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/throw_dice.rs)]
//! ```no_run
//! # #[cfg(feature = "ctrlc_handler")]
//! use teloxide::prelude::*;
//!
//! # #[cfg(feature = "ctrlc_handler")]
//! # #[tokio::main]
//! # async fn main() {
//! pretty_env_logger::init();
//! log::info!("Starting throw dice bot...");
//!
//! let bot = Bot::from_env();
//!
//! teloxide::repl(bot, |bot: Bot, msg: Message| async move {
//!     bot.send_dice(msg.chat.id).await?;
//!     Ok(())
//! })
//! .await;
//! # } #[cfg(not(feature = "ctrlc_handler"))] fn main(){}
//! ```
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/throw-dice.gif width=420px />
//!   </kbd>
//! </div>
//! 
//! ## Working with Updates and Messages
//! There're a great number of [update kinds](https://docs.rs/teloxide/latest/teloxide/types/enum.UpdateKind.html) and 
//! [message kinds](https://docs.rs/teloxide/latest/teloxide/types/enum.MessageKind.html) to work with! Usually it's essential to filter specific ones
//! and process them in *handler functions*. *Teloxide* provides some `filter methods` for `Update` and `Message` types in [UpdateFilterExt](https://docs.rs/teloxide/latest/teloxide/dispatching/trait.UpdateFilterExt.html) 
//! and [MessageFilterExt](https://docs.rs/teloxide/latest/teloxide/dispatching/trait.MessageFilterExt.html) traits respectively. In addition to filtering, these
//! methods will inject the appropriate type into your handler functions. For instance, if you use `Update::filter_message`, the `Message` object will be available as a parameter
//! at your handler functions. Analogously the use of `Message::filter_text` will inject the `String` object.
//! 
//! (Note: `filter_text` actually uses a function that returns Option<&str> value, so every filter_.. fn *always* return an `Owned` version of a  type)
//! 
//! Moreover, *filter_map* function can inject some object according to the schema flow. More in the example below!
//! 
//! Here is a quick example (filter text message and inject it's text into the handler function):
//! ```no_run
//! # #[cfg(feature = "ctrlc_handler")]
//! use teloxide::prelude::*;
//!
//! # #[cfg(feature = "ctrlc_handler")]
//! # #[tokio::main]
//! # async fn main() {
//! let bot = Bot::from_env();
//! let schema = Update::filter_message()
//!     // Inject the `User` object representing the author of an incoming 
//!     // message into every successive handler function
//!     .filter_map(|update: Update| update.user().cloned())
//!     .branch(
//!         // Use filter_text method of MessageFilterExt to accept 
//!         // only textual messages. Others will be ignored by this handler
//!         Message::filter_text().endpoint(process_text_message)
//!     );
//! 
//! Dispatcher::builder(bot, schema)
//!        .build()
//!        .dispatch()
//!        .await;
//!    Ok(())
//! })
//! .await;
//! # } #[cfg(not(feature = "ctrlc_handler"))] fn main(){}
//! 
//! /// Replies to the user's text messages
//! async fn process_text_message(bot: Bot, user: User, message_text: String) -> Result<(), teloxide::RequestError> {
//!     // The id of a chat with a user is the same as his telegram_id 
//!     // from the bot's perspective
//!     bot.send_message(user.id, format!("Hi! You sent: {message_text}"))
//!     Ok(())
//! }
//! ```
//! 
//! [Telegram bots]: https://telegram.org/blog/bot-revolution
//! [`async`/`.await`]: https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
//! [Rust]: https://www.rust-lang.org/

// This hack is used to cancel formatting for a Markdown table. See [1], [2], and [3].
//
// [1]: https://github.com/rust-lang/rustfmt/issues/4210
// [2]: https://github.com/rust-lang/rustfmt/issues/4787
// [3]: https://github.com/rust-lang/rust/issues/82768#issuecomment-803935643
#![cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("features.md")))]
// https://github.com/teloxide/teloxide/raw/master/media/teloxide-logo.svg doesn't work in html_logo_url, I don't know why.
#![doc(
    html_logo_url = "https://github.com/teloxide/teloxide/raw/master/media/teloxide-logo.png",
    html_favicon_url = "https://github.com/teloxide/teloxide/raw/master/teloxide-logo.png"
)]
// To properly build docs of this crate run
// ```console
// $ cargo docs --open
// ```
// (docs is an alias from `.cargo/config.toml`)
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_auto_cfg))]
#![forbid(unsafe_code)]
#![warn(rustdoc::broken_intra_doc_links)]
#![allow(clippy::match_bool)]
#![allow(clippy::redundant_pattern_matching)]
// https://github.com/rust-lang/rust-clippy/issues/7422
#![allow(clippy::nonstandard_macro_braces)]

#[cfg(feature = "ctrlc_handler")]
pub use repls::{repl, repl_with_listener};

#[cfg(feature = "ctrlc_handler")]
#[allow(deprecated)]
pub use repls::{commands_repl, commands_repl_with_listener};

pub mod dispatching;
pub mod error_handlers;
pub mod prelude;
#[cfg(feature = "ctrlc_handler")]
pub mod repls;
pub mod stop;
pub mod update_listeners;
pub mod utils;

#[doc(inline)]
pub use teloxide_core::*;

#[cfg(feature = "macros")]
pub use teloxide_macros as macros;

pub use dispatching::filter_command;
pub use dptree::{self, case as handler};

#[cfg(all(feature = "nightly", doctest))]
#[cfg_attr(feature = "nightly", cfg_attr(feature = "nightly", doc = include_str!("../../../README.md")))]
enum ReadmeDocTests {}

use teloxide_core::requests::ResponseResult;

/// A shortcut for `ResponseResult::Ok(val)`.
pub fn respond<T>(val: T) -> ResponseResult<T> {
    ResponseResult::Ok(val)
}
