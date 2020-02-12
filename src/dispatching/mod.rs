//! Updates dispatching.
//!
//! The key type here is [`Dispatcher`]. It encapsulates [`Bot`], handlers for
//! [11 update kinds] (+ for [`Update`]) and [`ErrorHandler`] for them. When
//! [`Update`] is received from Telegram, the following steps are executed:
//!
//!  1. It is supplied into an appropriate handler (the first ones is those who
//! accept [`Update`]).
//!  2. If a handler failed, invoke [`ErrorHandler`] with the corresponding
//! error.
//!  3. If a handler has returned [`DispatcherHandlerResult`] with `None`,
//! terminate the pipeline, otherwise supply an update into the next handler
//! (back to step 1).
//!
//! The pipeline is executed until either all the registered handlers were
//! executed, or one of handlers has terminated the pipeline. That's simple!
//!
//!  1. Note that handlers implement [`CtxHandler`], which means that you are
//! able to supply [`DialogueDispatcher`] as a handler, since it implements
//! [`CtxHandler`] too!
//!  2. Note that you don't always need to return [`DispatcherHandlerResult`]
//! explicitly, because of automatic conversions. Just return `Result<(), E>` if
//! you want to terminate the pipeline (see the example below).
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
//! Dispatcher::<RequestError>::new(Bot::new("MyAwesomeToken"))
//!     .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
//!         ctx.answer("pong").send().await?;
//!         Ok(())
//!     })
//!     .dispatch()
//!     .await;
//! # }
//! ```
//!
//! For a bit more complicated example, please see [examples/dialogue_bot].
//!
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [11 update kinds]: crate::types::UpdateKind
//! [`Update`]: crate::types::Update
//! [`ErrorHandler`]: crate::dispatching::ErrorHandler
//! [`CtxHandler`]: crate::dispatching::CtxHandler
//! [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
//! [`DispatcherHandlerResult`]: crate::dispatching::DispatcherHandlerResult
//! [`Bot`]: crate::Bot
//! [examples/dialogue_bot]: https://github.com/teloxide/teloxide/tree/dev/examples/dialogue_bot

mod ctx_handlers;
pub mod dialogue;
mod dispatcher;
mod dispatcher_handler_ctx;
mod dispatcher_handler_result;
mod error_handlers;
pub mod update_listeners;

pub use ctx_handlers::CtxHandler;
pub use dispatcher::Dispatcher;
pub use dispatcher_handler_ctx::DispatcherHandlerCtx;
pub use dispatcher_handler_result::DispatcherHandlerResult;
pub use error_handlers::{
    ErrorHandler, IgnoringErrorHandler, IgnoringErrorHandlerSafe,
    LoggingErrorHandler,
};
