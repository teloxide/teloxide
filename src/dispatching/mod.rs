//! Updates dispatching.
//!
//! The key type here is [`Dispatcher`]. It encapsulates middlewares, handlers
//! for [10 update kinds], and [`ErrorHandler`] for them. When [`Update`] is
//! received from Telegram, the following steps are executed:
//!
//!  1. It is supplied into all registered middlewares.
//!  2. It is supplied to an appropriate handler.
//!  3. If a handler has returned an error, the error is supplied into an error
//! handler.
//!
//! That's simple!
//!
//! Note that handlers implement [`CtxHandler`], which means that you are able
//! to supply [`SessionDispatcher`] as a handler, since it implements
//! [`CtxHandler`] too!
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
//!     .message_handler(|ctx: DispatcherHandlerCtx<Message>| async move {
//!         ctx.answer("pong").send().await?;
//!     })
//!     .dispatch()
//!     .await;
//! # }
//! ```
//!
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [10 update kinds]: crate::types::UpdateKind
//! [`Update`]: crate::types::Update
//! [`ErrorHandler`]: crate::dispatching::ErrorHandler
//! [`CtxHandler`]: crate::dispatching::CtxHandler
//! [`SessionDispatcher`]: crate::dispatching::SessionDispatcher

mod ctx_handlers;
pub mod dialogue;
mod dispatcher;
mod dispatcher_handler_ctx;
mod error_handlers;
mod middleware;
pub mod update_listeners;

pub use ctx_handlers::CtxHandler;
pub use dispatcher::Dispatcher;
pub use dispatcher_handler_ctx::DispatcherHandlerCtx;
pub use error_handlers::{
    ErrorHandler, IgnoringErrorHandler, IgnoringErrorHandlerSafe,
    LoggingErrorHandler,
};
pub use middleware::Middleware;
