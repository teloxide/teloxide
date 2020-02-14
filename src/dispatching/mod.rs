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
//! # Examples
//! ### The ping-pong bot
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main_() {
//! use teloxide::prelude::*;
//!
//! // Setup logging here...
//!
//! // Create a dispatcher with a single message handler that answers "pong"
//! // to each incoming message.
//! Dispatcher::<RequestError>::new(Bot::from_env())
//!     .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
//!         ctx.answer("pong").send().await?;
//!         Ok(())
//!     })
//!     .dispatch()
//!     .await;
//! # }
//! ```
//!
//! [Full](https://github.com/teloxide/teloxide/blob/dev/examples/ping_pong_bot/)
//!
//! ### Multiple handlers
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main_() {
//! use teloxide::prelude::*;
//!
//! // Create a dispatcher with multiple handlers of different types. This will
//! // print One! and Two! on every incoming UpdateKind::Message.
//! Dispatcher::<RequestError>::new(Bot::from_env())
//!     // This is the first UpdateKind::Message handler, which will be called
//!     // after the Update handler below.
//!     .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
//!         log::info!("Two!");
//!         DispatcherHandlerResult::next(ctx.update, Ok(()))
//!     })
//!     // Remember: handler of Update are called first.
//!     .update_handler(&|ctx: DispatcherHandlerCtx<Update>| async move {
//!         log::info!("One!");
//!         DispatcherHandlerResult::next(ctx.update, Ok(()))
//!     })
//!     // This handler will be called right after the first UpdateKind::Message
//!     // handler, because it is registered after.
//!     .message_handler(&|_ctx: DispatcherHandlerCtx<Message>| async move {
//!         // The same as DispatcherHandlerResult::exit(Ok(()))
//!         Ok(())
//!     })
//!     // This handler will never be called, because the UpdateKind::Message
//!     // handler above terminates the pipeline.
//!     .message_handler(&|ctx: DispatcherHandlerCtx<Message>| async move {
//!         log::info!("This will never be printed!");
//!         DispatcherHandlerResult::next(ctx.update, Ok(()))
//!     })
//!     .dispatch()
//!     .await;
//!
//! // Note: if this bot receive, for example, UpdateKind::ChannelPost, it will
//! // only print "One!", because the UpdateKind::Message handlers will not be
//! // called.
//! # }
//! ```
//!
//! [Full](https://github.com/teloxide/teloxide/blob/dev/examples/miltiple_handlers_bot/)
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
