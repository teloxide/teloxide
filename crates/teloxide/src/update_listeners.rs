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

use std::pin::Pin;

use futures::{Future, Stream};

use crate::{
    stop::StopToken,
    types::{AllowedUpdate, Update},
};

mod polling;

#[allow(deprecated)]
pub use self::polling::{polling, polling_default, Polling, PollingBuilder, PollingStream};

/// An update listener.
///
/// Implementors of this trait allow getting updates from Telegram. See
/// [module-level documentation] for more.
///
/// [module-level documentation]: mod@self
pub trait UpdateListener {
    type SetupErr;

    /// Error that can be returned from the [`Stream`]
    ///
    /// [`Stream`]: UpdateListener::Stream
    type StreamErr;

    /// The stream of updates from Telegram.
    // NB: `Send` is not strictly required here, but it makes it easier to return
    //     `impl AsUpdateStream` and also you want `Send` streams almost (?) always
    //     anyway.
    type Stream<'a>: Stream<Item = Result<Update, Self::StreamErr>> + Send + 'a
    where
        Self: 'a;

    /// Creates the update [`Stream`].
    ///
    /// This function should also do all the necessary setup, and return an
    /// error if something goes wrong with it. For example for webhooks this
    /// should call `set_webhook`.
    ///
    /// [`Stream`]: AsUpdateStream::Stream
    fn listen(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Self::Stream<'_>, Self::SetupErr>> + Send + '_>>;

    /// Hint which updates should the listener listen for.
    ///
    /// For example [`polling()`] should send the hint as
    /// [`GetUpdates::allowed_updates`]
    ///
    /// Note: this is a very important method, without setting appropriate
    /// allowed updates, telegram will not send some update kinds.
    ///
    /// [`GetUpdates::allowed_updates`]:
    /// crate::payloads::GetUpdates::allowed_updates
    fn hint_allowed_updates(&mut self, hint: &mut dyn Iterator<Item = AllowedUpdate>);

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
}

#[inline(always)]
pub(crate) const fn assert_update_listener<L>(listener: L) -> L
where
    L: UpdateListener,
{
    listener
}
