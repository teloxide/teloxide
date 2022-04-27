//! An update dispatching model based on [`dptree`].
//!
//! In teloxide, updates are dispatched by a pipleine. The central type is
//! [`dptree::Handler`] -- it represents a handler of an update; since the API
//! is highly declarative, you can combine handlers with each other via such
//! methods as [`dptree::Handler::chain`] and [`dptree::Handler::branch`]. The
//! former method pipes one handler to another one, whilst the latter creates a
//! new node, as communicated by the name. For more information, please refer to
//! the documentation of [`dptree`].
//!
//! The pattern itself is called [chain of responsibility], a well-known design
//! technique across OOP developers. But unlike typical object-oriented design,
//! we employ declarative FP-style functions like [`dptree::filter`],
//! [`dptree::filter_map`], and [`dptree::endpoint`]; these functions create
//! special forms of [`dptree::Handler`]; for more information, please refer to
//! their respective documentation. Each of these higher-order functions accept
//! a closure that is made into a handler -- this closure can take any
//! additional parameters, which must be supplied while creating [`Dispatcher`]
//! (see [`DispatcherBuilder::dependencies`]).
//!
//! The [`Dispatcher`] type puts all these things together: it only provides
//! [`Dispatcher::dispatch`] and a handful of other methods. Once you call
//! `.dispatch()`, it will retrieve updates from the Telegram server and pass
//! them to your handler, which is a parameter of [`Dispatcher::builder`].
//!
//! Let us look at a simple example:
//!
//!
//! ([Full](https://github.com/teloxide/teloxide/blob/master/examples/shared_state.rs))
//!
//! ```no_run
//! use std::sync::atomic::{AtomicU64, Ordering};
//!
//! use once_cell::sync::Lazy;
//! use teloxide::prelude::*;
//!
//! static MESSAGES_TOTAL: Lazy<AtomicU64> = Lazy::new(AtomicU64::default);
//!
//! # #[tokio::main]
//! # async fn main() {
//! pretty_env_logger::init();
//! log::info!("Starting shared state bot...");
//!
//! let bot = Bot::from_env().auto_send();
//!
//! let handler = Update::filter_message().branch(dptree::endpoint(
//!     |msg: Message, bot: AutoSend<Bot>| async move {
//!         let previous = MESSAGES_TOTAL.fetch_add(1, Ordering::Relaxed);
//!         bot.send_message(msg.chat.id, format!("I received {} messages in total.", previous))
//!             .await?;
//!         respond(())
//!     },
//! ));
//!
//! Dispatcher::builder(bot, handler).build().setup_ctrlc_handler().dispatch().await;
//! # }
//! ```
//!
//!  1. First, we create the bot: `let bot = Bot::from_env().auto_send()`.
//!  2. Then we construct an update handler. While it is possible to handle all
//! kinds of [`crate::types::Update`], here we are only interested in
//! [`crate::types::Message`]: [`UpdateFilterExt::filter_message`] create a
//! handler object which filters all messages out of a generic update.
//!  3. By doing `.branch(dptree::endpoint(...))`, we set up a custom handling
//! closure that receives `msg: Message` and `bot: AutoSend<Bot>`. There are
//! called dependencies: `msg` is supplied by
//! [`UpdateFilterExt::filter_message`], while `bot` is supplied by
//! [`Dispatcher`].
//!
//! That being said, if we receive a message, the dispatcher will call our
//! handler, but if we receive something other than a message (e.g., a channel
//! post), you will see an unhandled update notice in your terminal.
//!
//! This is a very limited example of update pipelining facilities. In more
//! involved scenarios, there are multiple branches and chains; if one element
//! of a chain fails to handle an update, the update will be passed forwards; if
//! no handler succeeds at handling the update, [`Dispatcher`] will invoke a
//! default handler set up via [`DispatcherBuilder::default_handler`].
//!
//! Update pipelining provides several advantages over the typical `match
//! (update.kind) { ... }` approach:
//!
//!  1. It supports _extension_: e.g., you
//! can define extension filters or some other handlers and then combine them in
//! a single place, thus facilitating loose coupling.
//!  2. Pipelining exhibits a natural syntax for expressing message processing.
//!  3. Lastly, it provides a primitive form of [dependency injection (DI)],
//! which allows you to deal with such objects as a bot and various update types
//! easily.
//!
//! For a more involved example, see [`examples/dispatching_features.rs`](https://github.com/teloxide/teloxide/blob/master/examples/dispatching_features.rs).
//!
//! TODO: explain a more involved example with multiple branches.
//!
//! [chain of responsibility]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
//! [dependency injection (DI)]: https://en.wikipedia.org/wiki/Dependency_injection

#[cfg(all(feature = "ctrlc_handler"))]
pub mod repls;

pub mod dialogue;
mod dispatcher;
mod distribution;
mod filter_ext;
mod handler_description;
mod handler_ext;
mod handler_factory;
pub mod stop_token;
pub mod update_listeners;

pub use crate::utils::shutdown_token::{IdleShutdownError, ShutdownToken};
pub use dispatcher::{Dispatcher, DispatcherBuilder, UpdateHandler};
pub use distribution::DefaultKey;
pub use filter_ext::{MessageFilterExt, UpdateFilterExt};
pub use handler_description::DpHandlerDescription;
pub use handler_ext::{filter_command, HandlerExt};
#[allow(deprecated)]
pub use handler_factory::HandlerFactory;
