//! Updates dispatching.
//!
//! The key type here is [`Dispatcher`]. It encapsulates [`Bot`] and handlers
//! for [all the update kinds].
//!
//! Every handler accept [`tokio::sync::mpsc::UnboundedReceiver`] (the RX halve
//! of an asynchronous channel). Inside a body of your handler, you typically
//! asynchronously concurrently iterate through updates like this:
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
//! Since they implement [`DispatcherHandler`] too.
//!
//! [See the examples](https://github.com/teloxide/teloxide/tree/master/examples).
//!
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [all the update kinds]: crate::types::UpdateKind
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
pub(crate) mod repls;
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
