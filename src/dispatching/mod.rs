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
//! appropriate handler. That's simple!
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
//!
//! [Full](https://github.com/teloxide/teloxide/blob/master/examples/ping_pong_bot/)
//!
//! For a bit more complicated example, please see [examples/dialogue_bot].
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
mod dispatcher_handler_ctx;
mod error_handlers;
pub mod update_listeners;

pub use dispatcher::Dispatcher;
pub use dispatcher_handler::DispatcherHandler;
pub use dispatcher_handler_ctx::DispatcherHandlerCtx;
pub use error_handlers::{
    ErrorHandler, IgnoringErrorHandler, IgnoringErrorHandlerSafe,
    LoggingErrorHandler,
};
use tokio::sync::mpsc::UnboundedReceiver;

/// A type of a stream, consumed by [`Dispatcher`]'s handlers.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub type DispatcherHandlerRx<Upd> =
    UnboundedReceiver<DispatcherHandlerCtx<Upd>>;
