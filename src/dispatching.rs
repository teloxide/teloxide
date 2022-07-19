//! An update dispatching model based on [`dptree`].
//!
//! In teloxide, update dispatching is declarative: it takes the form of a
//! [chain of responsibility] pattern enriched with a number of combinator
//! functions, which together form an instance of the [`dptree::Handler`] type.
//!
//! Let us look at this simple example:
//!
//! [[`examples/purchase.rs`](https://github.com/teloxide/teloxide/blob/master/examples/purchase.rs)]
//! ```no_run
//! // Imports omitted...
//! # use teloxide::{
//! #     dispatching::{dialogue::InMemStorage, UpdateHandler},
//! #     prelude::*,
//! #     types::{InlineKeyboardButton, InlineKeyboardMarkup},
//! #     utils::command::BotCommands,
//! # };
//!
//! type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Start,
//!     ReceiveFullName,
//!     ReceiveProductChoice {
//!         full_name: String,
//!     },
//! }
//!
//! #[derive(BotCommands, Clone)]
//! #[command(rename = "lowercase", description = "These commands are supported:")]
//! enum Command {
//!     #[command(description = "display this text.")]
//!     Help,
//!     #[command(description = "start the purchase procedure.")]
//!     Start,
//!     #[command(description = "cancel the purchase procedure.")]
//!     Cancel,
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Setup is omitted...
//! #     pretty_env_logger::init();
//! #     log::info!("Starting purchase bot...");
//! #
//! #     let bot = Bot::from_env().auto_send();
//! #
//! #     Dispatcher::builder(bot, schema())
//! #         .dependencies(dptree::deps![InMemStorage::<State>::new()])
//! #         .build()
//! #         .setup_ctrlc_handler()
//! #         .dispatch()
//! #         .await;
//! }
//!
//! fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//!     let command_handler = teloxide::filter_command::<Command, _>()
//!         .branch(
//!             dptree::case![State::Start]
//!                 .branch(dptree::case![Command::Help].endpoint(help))
//!                 .branch(dptree::case![Command::Start].endpoint(start)),
//!         )
//!         .branch(dptree::case![Command::Cancel].endpoint(cancel));
//!
//!     let message_handler = Update::filter_message()
//!         .branch(command_handler)
//!         .branch(dptree::case![State::ReceiveFullName].endpoint(receive_full_name))
//!         .branch(dptree::endpoint(invalid_state));
//!
//!     let callback_query_handler = Update::filter_callback_query().branch(
//!         dptree::case![State::ReceiveProductChoice { full_name }]
//!             .endpoint(receive_product_selection),
//!     );
//!
//!     teloxide::dispatching::dialogue::enter::<Update, InMemStorage<State>, State, _>()
//!         .branch(message_handler)
//!         .branch(callback_query_handler)
//! }
//!
//! // Handler definitions omitted...
//!
//! async fn start(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn help(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn cancel(bot: AutoSend<Bot>, msg: Message, dialogue: MyDialogue) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn invalid_state(bot: AutoSend<Bot>, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn receive_full_name(
//!     bot: AutoSend<Bot>,
//!     msg: Message,
//!     dialogue: MyDialogue,
//! ) -> HandlerResult {
//!     todo!()
//! }
//!
//! async fn receive_product_selection(
//!     bot: AutoSend<Bot>,
//!     q: CallbackQuery,
//!     dialogue: MyDialogue,
//!     full_name: String,
//! ) -> HandlerResult {
//!     todo!()
//! }
//! ```
//!
//! The above code shows how to dispatch on different combinations of a state
//! and command _elegantly_. We give a top-bottom explanation of the function
//! `schema`, which constructs the main update handler:
//!
//!  - We call the [`dialogue::enter`] function to initiate dialogue
//! interaction. Then we call [`dptree::Handler::branch`] two times to form a
//! tree of responsibility of `message_handler` and `callback_query_handler`.
//!    - Inside `message_handler`, we use [`Update::filter_message`] as a filter
//! for incoming messages. Then we create a tree of responsibility again,
//! consisting of three branches with a similar structure.
//!    - Inside `callback_query_handler`, we use
//! [`Update::filter_callback_query`] as a filter and create one branch for
//! handling product selection.
//!
//! `a.branch(b)` roughly means "try to handle an update with `a`, then, if it
//! fails, try `b`". We use branching multiple times here, which is a natural
//! pattern for describing dispatching logic. We also use the [`dptree::case!`]
//! macro extensively, which acts as a filter on an enumeration: if it is of a
//! certain variant, it passes the variant's payload down the handler chain;
//! otherwise, it neglects an update. Note how we utilise this macro both for
//! `State` and `Command` in the same way!
//!
//! Finally, we plug the schema into [`Dispatcher`] like this:
//!
//! ```no_run
//! # #[tokio::main]
//! # async fn main() {
//! let bot = Bot::from_env().auto_send();
//!
//! Dispatcher::builder(bot, schema())
//!     .dependencies(dptree::deps![InMemStorage::<State>::new()])
//!     .build()
//!     .setup_ctrlc_handler()
//!     .dispatch()
//!     .await;
//! # }
//! ```
//!
//! In a call to [`DispatcherBuilder::dependencies`], we specify a list of
//! dependencies that all handlers will receive as parameters. Here, we only
//! specify an in-memory storage of dialogues needed for [`dialogue::enter`].
//! However, in production bots, you normally also pass a database connection,
//! configuration, and other stuff.
//!
//! All in all, [`dptree`] can be seen as an extensible alternative to pattern
//! matching, with support for [dependency injection (DI)] and a few other
//! useful features. See [`examples/dispatching_features.rs`] as a more involved
//! example.
//!
//! [`Update::filter_message`]: crate::types::Update::filter_message
//! [`Update::filter_callback_query`]: crate::types::Update::filter_callback_query
//! [chain of responsibility]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
//! [dependency injection (DI)]: https://en.wikipedia.org/wiki/Dependency_injection
//! [`examples/dispatching_features.rs`]: https://github.com/teloxide/teloxide/blob/master/examples/dispatching_features.rs

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
