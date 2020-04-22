//! Updates dispatching.
//!
//! teloxide has no dispatcher like in other Telegram libraries, instead you
//! apply a series of transformations to updates to dispatch them. To get these
//! updates, you need an [update listener].
//!
//! [`StreamExt`] is a very useful trait here: it provides common methods for
//! working with streams.
//!
//! # Examples
//! ## The ping-pong bot
//! This bot just answers "pong" to each incoming message:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/src/main.rs))
//! ```no_run
//! use teloxide::prelude::*;
//!
//! #[tokio::main]
//! async fn main_() {
//!     teloxide::enable_logging!();
//!     log::info!("Starting ping_pong_bot!");
//!
//!     let bot = Bot::from_env();
//!
//!     polling_default(bot)
//!         .basic_config()
//!         .for_each_concurrent(None, |message| async move {
//!             message.answer("pong").send().await.log_on_error().await;
//!         })
//!         .await;
//! }
//! ```
//!
//! <div align="center">
//!   <kbd>
//!     <img src=https://github.com/teloxide/teloxide/raw/master/media/PING_PONG_BOT.png width="600" />
//!   </kbd>
//! </div>
//!
//! [See more examples](https://github.com/teloxide/teloxide/tree/master/examples).
//!
//! [update listener]: crate::dispatching::update_listeners::UpdateListener
//! [`StreamExt`]: crate::dispatching::StreamExt

pub mod dialogue;
mod stream_ext;
pub mod update_listeners;
mod update_with_cx;

pub use stream_ext::StreamExt;
pub use update_with_cx::UpdateWithCx;
