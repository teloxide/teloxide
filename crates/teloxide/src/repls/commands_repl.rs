use crate::{
    dispatching::{HandlerExt, UpdateFilterExt},
    error_handlers::LoggingErrorHandler,
    requests::{Requester, ResponseResult},
    types::Update,
    update_listeners::{self, UpdateListener},
    utils::command::BotCommands,
};
use dptree::di::{DependencyMap, Injectable};
use futures::future::BoxFuture;
use std::fmt::Debug;

/// A [REPL] for commands.
///
/// REPLs are meant only for simple bots and rapid prototyping. If you need to
/// supply dependencies or describe more complex dispatch logic, please use
/// [`Dispatcher`]. See also: ["Dispatching or
/// REPLs?"](../index.html#dispatching-or-repls).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
///
/// All errors from the handler and update listener will be logged.
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
///
/// This trait extends your [`BotCommands`] type with REPL facilities.
///
/// ## Signatures
///
/// Don't be scared by many trait bounds in the signatures, in essence they
/// require:
///
/// 1. `bot` is a bot, client for the Telegram bot API. It is represented via
///    the [`Requester`] trait.
/// 2. `handler` is an `async` function that takes arguments from
///    [`DependencyMap`] (see below) and returns [`ResponseResult`].
/// 3. `listener` is something that takes updates from a Telegram server and
///    implements [`UpdateListener`].
///
/// All the other requirements are about thread safety and data validity and can
/// be ignored for most of the time.
///
/// ## Handler arguments
///
/// `teloxide` provides the following types to the `handler`:
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
pub trait CommandReplExt {
    /// A REPL for commands.
    ///
    /// See [`CommandReplExt`] for more details.
    #[must_use]
    fn repl<'a, R, H, Args>(bot: R, handler: H) -> BoxFuture<'a, ()>
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::GetUpdates: Send,
        <R as Requester>::GetWebhookInfo: Send,
        <R as Requester>::GetMe: Send,
        <R as Requester>::DeleteWebhook: Send,
        H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static;

    /// A REPL for commands with a custom [`UpdateListener`].
    ///
    /// See [`CommandReplExt`] for more details.
    #[must_use]
    fn repl_with_listener<'a, R, H, L, Args>(bot: R, handler: H, listener: L) -> BoxFuture<'a, ()>
    where
        H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
        L: UpdateListener + Send + 'a,
        L::Err: Debug + Send + 'a,
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::GetMe: Send;
}

#[cfg(feature = "ctrlc_handler")]
impl<Cmd> CommandReplExt for Cmd
where
    Cmd: BotCommands + Send + Sync + 'static,
{
    fn repl<'a, R, H, Args>(bot: R, handler: H) -> BoxFuture<'a, ()>
    where
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::GetUpdates: Send,
        <R as Requester>::GetWebhookInfo: Send,
        <R as Requester>::GetMe: Send,
        <R as Requester>::DeleteWebhook: Send,
        H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
    {
        let cloned_bot = bot.clone();

        Box::pin(async move {
            Self::repl_with_listener(
                bot,
                handler,
                update_listeners::polling_default(cloned_bot).await,
            )
            .await
        })
    }

    fn repl_with_listener<'a, R, H, L, Args>(bot: R, handler: H, listener: L) -> BoxFuture<'a, ()>
    where
        H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
        L: UpdateListener + Send + 'a,
        L::Err: Debug + Send + 'a,
        R: Requester + Clone + Send + Sync + 'static,
        <R as Requester>::GetMe: Send,
    {
        use crate::dispatching::Dispatcher;

        // Other update types are of no interest to use since this REPL is only for
        // commands. See <https://github.com/teloxide/teloxide/issues/557>.
        let ignore_update = |_upd| Box::pin(async {});

        Box::pin(async move {
            Dispatcher::builder(
                bot,
                Update::filter_message().filter_command::<Cmd>().endpoint(handler),
            )
            .default_handler(ignore_update)
            .enable_ctrlc_handler()
            .build()
            .dispatch_with_listener(
                listener,
                LoggingErrorHandler::with_custom_text("An error from the update listener"),
            )
            .await
        })
    }
}
