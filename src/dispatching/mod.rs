//! Updates dispatching.
//!
//! The key type here is [`Dispatcher`]. It encapsulates [`UpdateListener`], a
//! handler of errors, and handlers for [10 update kinds]. When [`Update`] is
//! received from Telegram, it is supplied to an appropriate handler, and if a
//! handler has returned an error, the error is supplied into an error handler.
//! That's simple!
//!
//! All the handlers are of type [`AsyncHandler`]. It's like a [first-class
//! construction] in this module, because:
//!  1. It is implemented for [`SessionDispatcher`], which itself accepts
//! [`AsyncHandler`].
//!  2. It is implemented for [`LoggingHandler`], [`IgnoringHandler`], and
//! [`IgnoringHandlerSafe`].
//!  3. It is implemented even [for asynchronous functions].
//!  4. You can use [`AsyncHandler`]s as error handlers.
//!  5. More...
//!
//! ## Examples
//! The ping-pong bot ([full](https://github.com/teloxide/teloxide/blob/dev/examples/ping_pong_bot/)):
//!
//! ```
//! # #[tokio::main]
//! # async fn main_() {
//! use teloxide::prelude::*;
//!
//! // Setup logging here...
//!
//! Dispatcher::new(Bot::new("MyAwesomeToken"))
//!     .message_handler(|ctx: HandlerCtx<Message>| async move {
//!         ctx.reply("pong").await
//!     })
//!     .dispatch()
//!     .await;
//! # }
//! ```
//!
//! [first-class construction]: https://stackoverflow.com/questions/646794/what-is-a-first-class-programming-construct
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
//! [10 update kinds]: crate::types::UpdateKind
//! [`Update`]: crate::types::Update
//! [`AsyncHandler`]: crate::dispatching::AsyncHandler
//! [`LoggingHandler`]: crate::dispatching::LoggingHandler
//! [`IgnoringHandler`]: crate::dispatching::IgnoringHandler
//! [`IgnoringHandlerSafe`]: crate::dispatching::IgnoringHandlerSafe
//! [for asynchronous functions]: crate::dispatching::AsyncHandler
//! [`SessionDispatcher`]: crate::dispatching::session::SessionDispatcher

mod async_handlers;
mod dispatcher;
mod handler_ctx;
pub mod session;
pub mod update_listeners;

pub use async_handlers::{
    AsyncHandler, IgnoringHandler, IgnoringHandlerSafe, LoggingHandler,
};
pub use dispatcher::Dispatcher;
pub use handler_ctx::HandlerCtx;
