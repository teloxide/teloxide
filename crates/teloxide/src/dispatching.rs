//! An update dispatching model based on [`dptree`].
//!
//! In `teloxide`, update dispatching is declarative: it takes the form of a
//! [chain of responsibility] pattern enriched with a number of combinator
//! functions, which together form an instance of the [`dptree::Handler`] type.
//!
//! Take [`examples/purchase.rs`] as an example of dispatching logic. First, we
//! define a type named `State` to represent the current state of a dialogue:
//!
//! ```no_run
//! #[derive(Clone, Default)]
//! pub enum State {
//!     #[default]
//!     Start,
//!     ReceiveFullName,
//!     ReceiveProductChoice {
//!         full_name: String,
//!     },
//! }
//! ```
//!
//! Then, we define a type `Command` to represent user commands such as
//! `/start` or `/help`:
//!
//! ```no_run
//! # #[cfg(feature = "macros")] {
//! # use teloxide::utils::command::BotCommands;
//! #[derive(BotCommands, Clone)]
//! #[command(rename_rule = "lowercase", description = "These commands are supported:")]
//! enum Command {
//!     #[command(description = "display this text.")]
//!     Help,
//!     #[command(description = "start the purchase procedure.")]
//!     Start,
//!     #[command(description = "cancel the purchase procedure.")]
//!     Cancel,
//! }
//! # }
//! ```
//!
//! Now the key question: how to elegantly dispatch on different combinations of
//! `State`, `Command`, and Telegram updates? -- i.e., we may want to execute
//! specific endpoints only in response to specific user commands and while we
//! are in a given dialogue state (and possibly under other circumstances!). The
//! solution is to use [`dptree`]:
//!
//! ```no_run
//! # #[cfg(feature = "macros")] {
//! # // That's a lot of context needed to compile this, oof
//! # use teloxide::dispatching::{UpdateHandler, UpdateFilterExt, dialogue, dialogue::InMemStorage};
//! # use teloxide::utils::command::BotCommands;
//! # use teloxide::types::Update;
//! # #[derive(Clone, Default)] pub enum State { #[default] Start, ReceiveFullName, ReceiveProductChoice { full_name: String } }
//! # #[derive(BotCommands, Clone)] enum Command { Help, Start, Cancel }
//! # type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//! # async fn help() -> HandlerResult { Ok(()) }
//! # async fn start() -> HandlerResult { Ok(()) }
//! # async fn cancel() -> HandlerResult { Ok(()) }
//! # async fn receive_full_name() -> HandlerResult { Ok(()) }
//! # async fn invalid_state() -> HandlerResult { Ok(()) }
//! # async fn receive_product_selection() -> HandlerResult { Ok(()) }
//! #
//! fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
//!     use dptree::case;
//!
//!     let command_handler = teloxide::filter_command::<Command, _>()
//!         .branch(
//!             case![State::Start]
//!                 .branch(case![Command::Help].endpoint(help))
//!                 .branch(case![Command::Start].endpoint(start)),
//!         )
//!         .branch(case![Command::Cancel].endpoint(cancel));
//!
//!     let message_handler = Update::filter_message()
//!         .branch(command_handler)
//!         .branch(case![State::ReceiveFullName].endpoint(receive_full_name))
//!         .branch(dptree::endpoint(invalid_state));
//!
//!     let callback_query_handler = Update::filter_callback_query().branch(
//!         case![State::ReceiveProductChoice { full_name }].endpoint(receive_product_selection),
//!     );
//!
//!     dialogue::enter::<Update, InMemStorage<State>, State, _>()
//!         .branch(message_handler)
//!         .branch(callback_query_handler)
//! }
//! # }
//! ```
//!
//! The overall logic should be clear. Throughout the above example, we use
//! several techniques:
//!
//!  - **Branching:** `a.branch(b)` roughly means "try to handle an update with
//!    `a`, then, if it neglects the update, try `b`".
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
//! Finally, we define our endpoints:
//!
//! ```no_run
//! # use teloxide::Bot;
//! # use teloxide::types::{Message, CallbackQuery};
//! # use teloxide::dispatching::dialogue::{InMemStorage, Dialogue};
//! # enum State{}
//! #
//! type MyDialogue = Dialogue<State, InMemStorage<State>>;
//! type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
//!
//! async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//! async fn help(bot: Bot, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//! async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//! async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//! async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//!     todo!()
//! }
//! async fn receive_product_selection(
//!     bot: Bot,
//!     dialogue: MyDialogue,
//!     full_name: String, // Available from `State::ReceiveProductChoice`.
//!     q: CallbackQuery,
//! ) -> HandlerResult {
//!     todo!()
//! }
//! ```
//!
//! Each parameter is supplied as a dependency by `teloxide`. In particular:
//!  - `bot: Bot` comes from the dispatcher (see below)
//!  - `msg: Message` comes from [`Update::filter_message`]
//!  - `q: CallbackQuery` comes from [`Update::filter_callback_query`]
//!  - `dialogue: MyDialogue` comes from [`dialogue::enter`]
//!  - `full_name: String` comes from `dptree::case![State::ReceiveProductChoice
//!    { full_name }]`
//!
//! Inside `main`, we plug the schema into [`Dispatcher`] like this:
//!
//! ```no_run
//! # #[cfg(feature = "ctrlc_handler")] {
//! # use teloxide::Bot;
//! # use teloxide::requests::RequesterExt;
//! # use teloxide::dispatching::{Dispatcher, dialogue::InMemStorage};
//! # enum State {}
//! # fn schema() -> teloxide::dispatching::UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> { teloxide::dptree::entry() }
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::from_env();
//!
//!     Dispatcher::builder(bot, schema())
//!         .dependencies(dptree::deps![InMemStorage::<State>::new()])
//!         .enable_ctrlc_handler()
//!         .build()
//!         .dispatch()
//!         .await;
//! }
//! # }
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
//! ## Dispatching or REPLs?
//!
//! The difference between dispatching and the REPLs ([`crate::repl`] & co) is
//! that dispatching gives you a greater degree of flexibility at the cost of a
//! bit more complicated setup.
//!
//! Here are things that dispatching can do, but REPLs can't:
//!  - Handle different kinds of [`Update`]
//!  - [Pass dependencies] to handlers
//!  - Disable a [default Ctrl-C handling]
//!  - Control your [default] and [error] handlers
//!  - Use [dialogues]
//!  - Use [`dptree`]-related functionality
//!  - Probably more
//!
//! Thus, REPLs are good for simple bots and rapid prototyping, but for more
//! involved scenarios, we recommend using dispatching over REPLs.
//!
//! [Pass dependencies]: DispatcherBuilder#method.dependencies
//! [default Ctrl-C handling]: DispatcherBuilder#method.enable_ctrlc_handler
//! [default]: DispatcherBuilder#method.default_handler
//! [error]: DispatcherBuilder#method.error_handler
//! [dialogues]: dialogue
//! [`examples/purchase.rs`]: https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/purchase.rs
//! [`Update::filter_message`]: crate::types::Update::filter_message
//! [`Update::filter_callback_query`]: crate::types::Update::filter_callback_query
//! [chain of responsibility]: https://en.wikipedia.org/wiki/Chain-of-responsibility_pattern
//! [dependency injection (DI)]: https://en.wikipedia.org/wiki/Dependency_injection
//! [`examples/dispatching_features.rs`]: https://github.com/teloxide/teloxide/blob/master/crates/teloxide/examples/dispatching_features.rs
//! [`Update`]: crate::types::Update

pub mod dialogue;

mod dispatcher;
mod distribution;
mod filter_ext;
mod handler_description;
mod handler_ext;

pub use crate::utils::shutdown_token::{IdleShutdownError, ShutdownToken};
pub use dispatcher::{Dispatcher, DispatcherBuilder, UpdateHandler};
pub use distribution::DefaultKey;
pub use filter_ext::{MessageFilterExt, UpdateFilterExt};
pub use handler_description::DpHandlerDescription;
pub use handler_ext::{filter_command, HandlerExt};
