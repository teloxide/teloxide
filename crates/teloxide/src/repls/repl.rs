use crate::{
    dispatching::UpdateFilterExt,
    error_handlers::LoggingErrorHandler,
    requests::{Requester, ResponseResult},
    types::Update,
    update_listeners::{self, UpdateListener},
};
use dptree::di::{DependencyMap, Injectable};
use std::fmt::Debug;

/// A [REPL] for messages.
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
///
/// ## Handler arguments
///
/// `teloxide` provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the message without [`Me`].
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
pub async fn repl<R, H, Args>(bot: R, handler: H)
where
    R: Requester + Send + Sync + Clone + 'static,
    <R as Requester>::GetUpdates: Send,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
{
    try_repl(bot, handler).await.expect("try_repl failed")
}

/// Same as `repl` but returns a Result<_> instead of panicking
///
/// A [REPL] for messages.
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
///
/// ## Handler arguments
///
/// `teloxide` provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the message without [`Me`].
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
pub async fn try_repl<R, H, Args>(bot: R, handler: H) -> Result<(), R::Err>
where
    R: Requester + Send + Sync + Clone + 'static,
    <R as Requester>::GetUpdates: Send,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
{
    let cloned_bot = bot.clone();
    try_repl_with_listener(bot, handler, update_listeners::polling_default(cloned_bot).await).await
}

/// A [REPL] for messages, with a custom [`UpdateListener`].
//
///
//
#[doc = include_str!("preamble.md")]
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`UpdateListener`]: crate::update_listeners::UpdateListener
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
///
/// ## Handler arguments
///
/// `teloxide` provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the message without [`Me`].
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
pub async fn repl_with_listener<R, H, L, Args>(bot: R, handler: H, listener: L)
where
    R: Requester + Clone + Send + Sync + 'static,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
    L: UpdateListener + Send,
    L::Err: Debug,
{
    try_repl_with_listener(bot, handler, listener).await.expect("try_repl_with_listener failed")
}

/// Same as `repl_with_listener` but returns a Result<_> instead of panicking
///
/// A [REPL] for messages, with a custom [`UpdateListener`].
//
///
//
#[doc = include_str!("preamble.md")]
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`UpdateListener`]: crate::update_listeners::UpdateListener
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
///
/// ## Handler arguments
///
/// `teloxide` provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
///
/// Each of these types can be accepted as a handler parameter. Note that they
/// aren't all required at the same time: e.g., you can take only the bot and
/// the message without [`Me`].
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
pub async fn try_repl_with_listener<R, H, L, Args>(
    bot: R,
    handler: H,
    listener: L,
) -> Result<(), R::Err>
where
    R: Requester + Clone + Send + Sync + 'static,
    H: Injectable<DependencyMap, ResponseResult<()>, Args> + Send + Sync + 'static,
    L: UpdateListener + Send,
    L::Err: Debug,
{
    use crate::dispatching::Dispatcher;

    // Other update types are of no interest to use since this REPL is only for
    // messages. See <https://github.com/teloxide/teloxide/issues/557>.
    let ignore_update = |_upd| Box::pin(async {});

    Dispatcher::builder(bot, Update::filter_message().endpoint(handler))
        .default_handler(ignore_update)
        .enable_ctrlc_handler()
        .build()
        .try_dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await?;

    Ok(())
}

#[test]
fn repl_is_send() {
    let bot = crate::Bot::new("");
    let repl = crate::repl(bot, || async { Ok(()) });
    assert_send(&repl);

    fn assert_send(_: &impl Send) {}
}
