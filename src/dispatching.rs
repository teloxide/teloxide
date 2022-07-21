//! An update dispatching model based on [`dptree`].
//!
//! In teloxide, update dispatching is declarative: it takes the form of a
//! [chain of responsibility] pattern enriched with a number of combinator
//! functions, which together form an instance of the [`dptree::Handler`] type.
//!
//! Take [`examples/purchase.rs`] as an example of dispatching logic. First, we
//! define a type named `State` to represent the current state of a dialogue:
//!
//! ```ignore
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Start,
//!     ReceiveFullName,
//!     ReceiveProductChoice { full_name: String },
//! }
//! ```
//!
//! Then, we define a type `Command` to represent user commands such as
//! `/start` or `/help`:
//!
//! ```ignore
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
//! ```
//!
//! Now the key question: how to elegantly dispatch on different combinations of
//! `State`, `Command`, and Telegram updates? -- i.e., we may want to execute
//! specific endpoints only in response to specific user commands and while we
//! are in a given dialogue state (and possibly under other circumstances!). The
//! solution is to use [`dptree`]:
//!
//! ```ignore
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
//! ```
//!
//! The overall logic should be clear. Throughout the above example, we use
//! several techniques:
//!
//!  - **Branching:** `a.branch(b)` roughly means "try to handle an update with
//!    `a`, then, if it
//! neglects the update, try `b`".
//!  - **Pattern matching:** We also use the [`dptree::case!`] macro
//!    extensively, which acts as a filter on an enumeration: if it is of a
//!    certain variant, it passes the variant's payload down the handler chain;
//!    otherwise, it neglects an update.
//!  - **Endpoints:** To specify the final function to handle an update, we use
//!    [`dptree::Handler::endpoint`].
//!
//! Notice the clear and uniform code structure: regardless of the dispatch
//! criteria, we use the same program constructions. In future, you may want to
//! introduce your application-specific filters or data structures to match upon
//! -- no problem, reuse [`dptree::Handler::filter`], [`dptree::case!`], and
//! other combinators in the same way!
//!
//! Finally, we define our endpoints like this:
//!
//! ```ignore
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
//! Each parameter is supplied as a dependency by teloxide. In particular:
//!  - `bot: AutoSend<Bot>` comes from the dispatcher (see below);
//!  - `msg: Message` comes from [`Update::filter_message`];
//!  - `q: CallbackQuery` comes from [`Update::filter_callback_query`];
//!  - `dialogue: MyDialogue` comes from [`dialogue::enter`];
//!  - `full_name: String` comes from `dptree::case![State::ReceiveProductChoice
//!    { full_name }]`.
//!
//! Inside `main`, we plug the schema into [`Dispatcher`] like this:
//!
//! ```ignore
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::from_env().auto_send();
//!
//!     Dispatcher::builder(bot, schema())
//!         .dependencies(dptree::deps![InMemStorage::<State>::new()])
//!         .enable_ctrlc_handler()
//!         .build()
//!         .dispatch()
//!         .await;
//! }
//! ```
//!
//! In a call to [`DispatcherBuilder::dependencies`], we specify a list of
//! additional dependencies that all handlers will receive as parameters. Here,
//! we only specify an in-memory storage of dialogues needed for
//! [`dialogue::enter`]. However, in production bots, you normally also pass a
//! database connection, configuration, and other stuff.
//!
//! All in all, [`dptree`] can be seen as an extensible alternative to pattern
//! matching, with support for [dependency injection (DI)] and a few other
//! useful features. See [`examples/dispatching_features.rs`] as a more involved
//! example.
//!
//! [`examples/purchase.rs`]: https://github.com/teloxide/teloxide/blob/master/examples/purchase.rs
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
