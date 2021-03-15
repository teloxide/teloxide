//! Core part of the [`teloxide`] library.
//!
//! This library provides tools for making requests to the [Telegram Bot API]
//! (Currently, version `5.` is supported) with ease. The library is fully
//! asynchronouns and built using [`tokio`].
//!
//!```toml
//! teloxide_core = "0.1"
//! ```
//! _Compiler support: requires rustc 1.49+_
//!
//! ```
//! # #[cfg(feature = "auto_send")]
//! # async {
//! # let chat_id = 0;
//! use teloxide_core::{
//!     prelude::*,
//!     types::{DiceEmoji, ParseMode},
//! };
//!
//! let bot = Bot::from_env()
//!     .parse_mode(ParseMode::MarkdownV2)
//!     .auto_send();
//!
//! let me = bot.get_me().await?;
//!
//! bot.send_dice(chat_id).emoji(DiceEmoji::Dice).await?;
//! bot.send_message(
//!     chat_id,
//!     format!("Hi, my name is **{}** 👋", me.user.first_name),
//! )
//! .await?;
//! # Ok::<_, Box<dyn std::error::Error>>(()) };
//! ```
//!
//! <div align="center">
//!     <img src=https://user-images.githubusercontent.com/38225716/103929465-6b91e100-512e-11eb-826d-39b096f16548.gif />
//! </div>
//!
//! [`teloxide`]: https://docs.rs/teloxide
//! [Telegram Bot API]: https://core.telegram.org/bots/api
//! [`tokio`]: https://tokio.rs
//!
//! ## Cargo features
//!
//! - `native-tls` = use [`native-tls`] tls implementation (**enabled by
//!   default**)
//! - `rustls` — use [`rustls`] tls implementation
//! - `auto_send` — enables [`AutoSend`] bot adaptor
//! - `throttle` — enables [`Throttle`] bot adaptor
//! - `cache_me` — enables [`CacheMe`] bot adaptor
//! - `full` — enables all features except `nigthly`
//! - `nightly` — enables nigthly-only features, currently:
//!   - Removes some future boxing using `#![feature(type_alias_impl_trait)]`
//!   - Used to built docs (`#![feature(doc_cfg, doc_spotlight)]`)
//!
//! [`AutoSend`]: adaptors::AutoSend
//! [`Throttle`]: adaptors::Throttle
//! [`CacheMe`]: adaptors::CacheMe
//! [`native-tls`]: https://docs.rs/native-tls
//! [`rustls`]: https://docs.rs/rustls

#![doc(
    // FIXME(waffle): use github
    html_logo_url = "https://cdn.discordapp.com/attachments/224881373326999553/798598120760934410/logo.png",
    html_favicon_url = "https://cdn.discordapp.com/attachments/224881373326999553/798598120760934410/logo.png"
)]
#![forbid(unsafe_code)]
// we pass "--cfg docsrs" when building docs to add `This is supported on feature="..." only.`
//
// To properly build docs of this crate run
// ```console
// $ RUSTDOCFLAGS="--cfg docsrs -Znormalize-docs" cargo doc --open --all-features
// ```
#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_spotlight))]
#![cfg_attr(feature = "nightly", feature(type_alias_impl_trait))]
#![cfg_attr(feature = "full", deny(broken_intra_doc_links))]
//#![deny(missing_docs)]

// The internal helper macros.
#[macro_use]
mod local_macros;

pub use self::{
    bot::Bot,
    errors::{ApiError, DownloadError, RequestError},
};

pub mod adaptors;
pub mod net;
pub mod payloads;
pub mod prelude;
pub mod requests;
pub mod types;

// reexported
mod bot;
mod errors;

// implementation details
mod serde_multipart;
