use crate::{
    dispatching::{
        update_listeners, update_listeners::UpdateListener, HandlerExt, UpdateFilterExt,
    },
    error_handlers::LoggingErrorHandler,
    types::Update,
    utils::command::BotCommands,
};
use dptree::di::{DependencyMap, Injectable};
use std::{fmt::Debug, marker::PhantomData};
use teloxide_core::requests::Requester;

/// A [REPL] for commands.
///
/// All errors from an update listener and handler will be logged.
///
/// ## Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// ## Dependency requirements
///
///  - Those of [`HandlerExt::filter_command`].
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl<'a, R, Cmd, H, E, Args>(bot: R, handler: H, cmd: PhantomData<Cmd>)
where
    Cmd: BotCommands + Send + Sync + 'static,
    H: Injectable<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    R: Requester + Clone + Send + Sync + 'static,
    <R as Requester>::GetUpdates: Send,
    E: Debug + Send + Sync + 'static,
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

/// Like [`commands_repl`], but with a custom [`UpdateListener`].
///
/// All errors from an update listener and handler will be logged.
///
/// ## Caution
/// **DO NOT** use this function together with [`Dispatcher`] and other REPLs,
/// because Telegram disallow multiple requests at the same time from the same
/// bot.
///
/// ## Dependency requirements
///
///  - Those of [`HandlerExt::filter_command`].
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
/// [`commands_repl`]: crate::dispatching::repls::commands_repl()
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
#[cfg(feature = "ctrlc_handler")]
pub async fn commands_repl_with_listener<'a, R, Cmd, H, L, ListenerE, E, Args>(
    bot: R,
    handler: H,
    listener: L,
    _cmd: PhantomData<Cmd>,
) where
    Cmd: BotCommands + Send + Sync + 'static,
    H: Injectable<DependencyMap, Result<(), E>, Args> + Send + Sync + 'static,
    L: UpdateListener<ListenerE> + Send + 'a,
    ListenerE: Debug + Send + 'a,
    R: Requester + Clone + Send + Sync + 'static,
    E: Debug + Send + Sync + 'static,
{
    use crate::dispatching::Dispatcher;

    // Other update types are of no interest to use since this REPL is only for
    // commands. See <https://github.com/teloxide/teloxide/issues/557>.
    let ignore_update = |_upd| Box::pin(async {});

    let mut dispatcher = Dispatcher::builder(
        bot,
        Update::filter_message().filter_command::<Cmd>().branch(dptree::endpoint(handler)),
    )
    .default_handler(ignore_update)
    .build();

    #[cfg(feature = "ctrlc_handler")]
    dispatcher.setup_ctrlc_handler();

    // To make mutable var from immutable.
    let mut dispatcher = dispatcher;

    dispatcher
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
