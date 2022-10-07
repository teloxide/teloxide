use crate::{
    dispatching::{update_listeners, update_listeners::UpdateListener, UpdateFilterExt},
    error_handlers::LoggingErrorHandler,
    types::Update,
    RequestError,
};
use dptree::di::{DependencyMap, Injectable};
use std::fmt::Debug;
use teloxide_core::requests::Requester;

/// A [REPL] for messages.
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
/// 1. `bot` is a bot, client for the Telegram bot API
///    - in teloxide this is represented via a [`Requester`] trait
/// 2. `handler` is an async function that returns `Result<(), E>`
///    - Such that `E` can be printed with [`Debug`] formatting
///    - And all arguments can be extracted from [`DependencyMap`]
///       - Which is the same, as all arguments implementing `Send + Sync +
///         'static`
///
/// ## Handler arguments
///
/// Teloxide provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
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
    H: Injectable<DependencyMap, Result<(), RequestError>, Args> + Send + Sync + 'static,
{
    let cloned_bot = bot.clone();
    repl_with_listener(bot, handler, update_listeners::polling_default(cloned_bot).await).await;
}

/// A [REPL] for messages, with a custom [`UpdateListener`].
//
#[doc = include_str!("preamble.md")]
///
/// [REPL]: https://en.wikipedia.org/wiki/Read-eval-print_loop
/// [`UpdateListener`]: crate::dispatching::update_listeners::UpdateListener
///
/// ## Signature
///
/// Don't be scared by many trait bounds in the signature, in essence they
/// require:
///
/// 1. `bot` is a bot, client for the Telegram bot API
///    - in teloxide this is represented via a [`Requester`] trait
/// 2. `handler` is an async function that returns `Result<(), E>`
///    - Such that `E` can be printed with [`Debug`] formatting
///    - And all arguments can be extracted from [`DependencyMap`]
///       - Which is the same, as all arguments implementing `Send + Sync +
///         'static`
/// 3. `listener` is an [`UpdateListener`]
///
/// ## Handler arguments
///
/// Teloxide provides the following types to the `handler`:
/// - [`Message`]
/// - `R` (type of the `bot`)
/// - [`Me`]
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
    H: Injectable<DependencyMap, Result<(), RequestError>, Args> + Send + Sync + 'static,
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
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}

#[test]
fn repl_is_send() {
    let bot = crate::Bot::new("");
    let repl = crate::repl(bot, || async { Ok(()) });
    assert_send(&repl);

    fn assert_send(_: &impl Send) {}
}
