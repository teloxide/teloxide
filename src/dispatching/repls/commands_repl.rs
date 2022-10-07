use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, HandlerExt, UpdateFilterExt,
    },
    error_handlers::LoggingErrorHandler,
    requests::{Requester, ResponseResult},
    types::Update,
    utils::command::BotCommands,
};
use dptree::di::{DependencyMap, Injectable};
use std::{fmt::Debug, marker::PhantomData};

/// A [REPL] for commands.
//
///
//
#[doc = include_str!("preamble.md")]
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
///
/// ## Signature
///
/// Don't be scared by many trait bounds in the signature, in essence they
/// require:
///
/// 1. `bot` is a bot, client for the Telegram bot API. It is represented via
///    the [`Requester`] trait.
/// 2. `handler` is an `async` function that takes arguments from
///    [`DependencyMap`] (see below) and returns [`ResponseResult`].
/// 3. `cmd` is a type hint for your command enumeration
///    `MyCommand`: just write `MyCommand::ty()`. Note that `MyCommand` must
///    implement the [`BotCommands`] trait, typically via
///   `#[derive(BotCommands)]`.
///
/// All the other requirements are about thread safety and data validity and can
/// be ignored for most of the time.
///
/// ## Handler arguments
///
/// Teloxide provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - `Cmd` (type of the parsed command)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the command without [`Me`] and [`Message`].
///
/// [`Me`]: crate::types::Me
/// [`Message`]: crate::types::Message
///
/// ## Stopping
//
#[doc = include_str!("stopping.md")]
///
/// ## Caution
//
#[doc = include_str!("caution.md")]
///
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl<'a, R, Cmd, H, Args>(bot: R, handler: H, cmd: PhantomData<Cmd>)
where
    R: Requester + Clone + Send + Sync + 'static,
    <R as Requester>::GetUpdates: Send,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
    Cmd: BotCommands + Send + Sync + 'static,
{
    let cloned_bot = bot.clone();

    commands_repl_with_listener(
        bot,
        handler,
        update_listeners::polling_default(cloned_bot).await,
        cmd,
    )
    .await;
}

/// A [REPL] for commands, with a custom [`UpdateListener`].
//
///
//
#[doc = include_str!("preamble.md")]
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
///
/// ## Signature
///
/// Don't be scared by many trait bounds in the signature, in essence they
/// require:
///
/// 1. `bot` is a bot, client for the Telegram bot API. It is represented via
///    the [`Requester`] trait.
/// 2. `handler` is an `async` function that takes arguments from
///    [`DependencyMap`] (see below) and returns [`ResponseResult`].
/// 3. `listener` is something that takes updates from a Telegram server and
///    implements [`UpdateListener`].
/// 4. `cmd` is a type hint for your command enumeration `MyCommand`: just
///    write `MyCommand::ty()`. Note that `MyCommand` must implement the
///   [`BotCommands`] trait, typically via `#[derive(BotCommands)]`.
///
/// All the other requirements are about thread safety and data validity and can
/// be ignored for most of the time.
///
/// ## Handler arguments
///
/// Teloxide provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - `Cmd` (type of the parsed command)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the command without [`Me`] and [`Message`].
///
/// [`Me`]: crate::types::Me
/// [`Message`]: crate::types::Message
///
/// ## Stopping
//
#[doc = include_str!("stopping.md")]
///
/// ## Caution
//
#[doc = include_str!("caution.md")]
///
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl_with_listener<'a, R, Cmd, H, L, Args>(
    bot: R,
    handler: H,
    listener: L,
    cmd: PhantomData<Cmd>,
) where
    Cmd: BotCommands + Send + Sync + 'static,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
    L: UpdateListener + Send + 'a,
    L::Err: Debug + Send + 'a,
    R: Requester + Clone + Send + Sync + 'static,
{
    use crate::dispatching::Dispatcher;

    let _ = cmd;

    // Other update types are of no interest to use since this REPL is only for
    // commands. See <https://github.com/teloxide/teloxide/issues/557>.
    let ignore_update = |_upd| Box::pin(async {});

    Dispatcher::builder(bot, Update::filter_message().filter_command::<Cmd>().endpoint(handler))
        .default_handler(ignore_update)
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
