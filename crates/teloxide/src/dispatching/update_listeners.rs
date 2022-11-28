//! Receiving updates from Telegram.
//!
//! The key trait here is [`UpdateListener`]. You can get its implementation
//! from:
//!
//! - [`polling_default`] function, which returns a default long polling
//!   listener.
//! - [`polling`] function, which returns a long polling listener with your
//!   configuration.
//! - Various functions in the [`webhooks`] module that return webhook listeners
//!
//! And then you can extract updates from it or pass them directly to a
//! [`Dispatcher`].
//!
//! Telegram supports two ways of [getting updates]: [long polling] and
//! [webhooks]. For the former see [`polling`] and [`polling_default`], for the
//! latter see the [`webhooks`] module.
//!
//! [`UpdateListener`]: UpdateListener
//! [`polling_default`]: polling_default
//! [`polling`]: polling()
//! [`Dispatcher`]: crate::dispatching::Dispatcher
//! [`Box::get_updates`]: crate::requests::Requester::get_updates
//! [getting updates]: https://core.telegram.org/bots/api#getting-updates
//! [long polling]: https://en.wikipedia.org/wiki/Push_technology#Long_polling
//! [webhooks]: https://en.wikipedia.org/wiki/Webhook

/// Implementations of webhook update listeners - an alternative (to
/// [`fn@polling`]) way of receiving updates from telegram.
#[cfg(feature = "webhooks")]
pub mod webhooks;

use futures::Stream;

use std::time::Duration;

use crate::{
    stop::StopToken,
    types::{AllowedUpdate, Update},
};

mod polling;
mod stateful_listener;

#[allow(deprecated)]
pub use self::{
    polling::{polling, polling_default, Polling, PollingBuilder, PollingStream},
    stateful_listener::StatefulListener,
};

/// An update listener.
///
/// Implementors of this trait allow getting updates from Telegram. See
/// [module-level documentation] for more.
///
/// Some functions of this trait are located in the supertrait
/// ([`AsUpdateStream`]), see also:
/// - [`AsUpdateStream::Stream`]
/// - [`AsUpdateStream::as_stream`]
///
/// [module-level documentation]: mod@self
pub trait UpdateListener:
    for<'a> AsUpdateStream<'a, StreamErr = <Self as UpdateListener>::Err>
{
    /// The type of errors that can be returned from this listener.
    type Err;

    /// Returns a token which stops this listener.
    ///  
    /// The [`stop`] function of the token is not guaranteed to have an
    /// immediate effect. That is, some listeners can return updates even
    /// after [`stop`] is called (e.g.: because of buffering).
    ///
    /// [`stop`]: StopToken::stop
    ///
    /// Implementors of this function are encouraged to stop listening for
    /// updates as soon as possible and return `None` from the update stream as
    /// soon as all cached updates are returned.
    #[must_use = "This function doesn't stop listening, to stop listening you need to call `stop` \
                  on the returned token"]
    fn stop_token(&mut self) -> StopToken;

    /// Hint which updates should the listener listen for.
    ///
    /// For example [`polling()`] should send the hint as
    /// [`GetUpdates::allowed_updates`]
    ///
    /// Note however that this is a _hint_ and as such, it can be ignored. The
    /// listener is not guaranteed to only return updates which types are listed
    /// in the hint.
    ///
    /// [`GetUpdates::allowed_updates`]:
    /// crate::payloads::GetUpdates::allowed_updates
    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>) {
        let _ = hint;
    }

    /// The timeout duration hint.
    ///
    /// This hints how often dispatcher should check for a shutdown. E.g., for
    /// [`polling()`] this returns the [`timeout`].
    ///
    /// [`timeout`]: crate::payloads::GetUpdates::timeout
    ///
    /// If you are implementing this trait and not sure what to return from this
    /// function, just leave it with the default implementation.
    fn timeout_hint(&self) -> Option<Duration> {
        None
    }
}

/// [`UpdateListener`]'s supertrait/extension.
///
/// This trait is a workaround to not require GAT.
pub trait AsUpdateStream<'a> {
    /// Error that can be returned from the [`Stream`]
    ///
    /// [`Stream`]: AsUpdateStream::Stream
    // NB: This should be named differently to `UpdateListener::Err`, so that it's
    // unambiguous
    type StreamErr;

    /// The stream of updates from Telegram.
    // NB: `Send` is not strictly required here, but it makes it easier to return
    //     `impl AsUpdateStream` and also you want `Send` streams almost (?) always
    //     anyway.
    type Stream: Stream<Item = Result<Update, Self::StreamErr>> + Send + 'a;

    /// Creates the update [`Stream`].
    ///
    /// [`Stream`]: AsUpdateStream::Stream
    fn as_stream(&'a mut self) -> Self::Stream;
}

#[inline(always)]
pub(crate) fn assert_update_listener<L>(listener: L) -> L
where
    L: UpdateListener,
{
    listener
}
