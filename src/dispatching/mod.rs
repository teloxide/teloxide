//! Updates dispatching.
//!
//! The key type here is [`Dispatcher`]. It encapsulates [`Bot`] and handlers
//! for [the 11 update kinds].
//!
//! You can register a maximum of 11 handlers for [the 11 update kinds]. Every
//! handler accept [`tokio::sync::mpsc::UnboundedReceiver`] (the RX halve of an
//! asynchronous unbounded MPSC channel). Inside a body of your handler, you
//! typically asynchronously concurrently iterate through updates like this:
//!
//! ```
//! use teloxide::prelude::*;
//!
//! async fn handle_messages(rx: DispatcherHandlerRx<Message>) {
//!     rx.for_each_concurrent(None, |message| async move {
//!         dbg!(message);
//!     })
//!     .await;
//! }
//! ```
//!
//! When [`Update`] is received from Telegram, [`Dispatcher`] pushes it into an
//! appropriate handler, depending on its kind. That's simple!
//!
//! **Note** that handlers must implement [`DispatcherHandler`], which means
//! that:
//!  - You are able to supply [`DialogueDispatcher`] as a handler.
//!  - You are able to supply functions that accept
//!    [`tokio::sync::mpsc::UnboundedReceiver`] and return `Future<Output = ()`
//!    as a handler.
//!
//! Since they implement [`DispatcherHandler`] too!
//!
//! # Examples
//! ### The ping-pong bot
//! This bot has a single message handler, which answers "pong" to each incoming
//! message:
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/src/main.rs))
//! ```no_run
//! use teloxide::prelude::*;
//!
//! # #[tokio::main]
//! # async fn main_() {
//! teloxide::enable_logging!();
//! log::info!("Starting ping_pong_bot!");
//!
//! let bot = Bot::from_env();
//!
//! Dispatcher::new(bot)
//!     .messages_handler(|rx: DispatcherHandlerRx<Message>| {
//!         rx.for_each(|message| async move {
//!             message.answer("pong").send().await.log_on_error().await;
//!         })
//!     })
//!     .dispatch()
//!     .await;
//! # }
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
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [the 11 update kinds]: crate::types::UpdateKind
//! [`Update`]: crate::types::Update
//! [`ErrorHandler`]: crate::dispatching::ErrorHandler
//! [`DispatcherHandler`]: crate::dispatching::DispatcherHandler
//! [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
//! [`DispatcherHandlerResult`]: crate::dispatching::DispatcherHandlerResult
//! [`Bot`]: crate::Bot
//! [`tokio::sync::mpsc::UnboundedReceiver`]: https://docs.rs/tokio/0.2.11/tokio/sync/mpsc/struct.UnboundedReceiver.html
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/tree/master/examples/dialogue_bot

pub mod dialogue;
mod dispatcher;
mod dispatcher_handler;
mod dispatcher_handler_rx_ext;
pub mod update_listeners;
mod update_with_cx;

pub use dispatcher::Dispatcher;
pub use dispatcher_handler::DispatcherHandler;
pub use dispatcher_handler_rx_ext::DispatcherHandlerRxExt;
use tokio::sync::mpsc::UnboundedReceiver;
pub use update_with_cx::UpdateWithCx;

/// A type of a stream, consumed by [`Dispatcher`]'s handlers.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub type DispatcherHandlerRx<Upd> = UnboundedReceiver<UpdateWithCx<Upd>>;
