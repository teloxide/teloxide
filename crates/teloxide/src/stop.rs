//! Stopping asynchronous tasks, e.g., [listeners].
//!
//! [listeners]: crate::update_listeners

use std::{convert::Infallible, future::Future, pin::Pin, task};

use futures::future::{pending, AbortHandle, Abortable, Pending};

/// Create a new token/flag pair.
#[must_use]
pub fn mk_stop_token() -> (StopToken, StopFlag) {
    let (handle, reg) = AbortHandle::new_pair();
    let token = StopToken(handle);
    let flag = StopFlag(Abortable::new(pending(), reg));

    (token, flag)
}

/// A stop token which corresponds to a [`StopFlag`].
#[derive(Clone)]
pub struct StopToken(AbortHandle);

/// A flag which corresponds to [`StopToken`].
///
/// To know if the stop token was used you can either repeatedly call
/// [`is_stopped`] or use this type as a `Future`.
///
/// [`is_stopped`]: StopFlag::is_stopped
#[pin_project::pin_project]
#[derive(Clone)]
pub struct StopFlag(#[pin] Abortable<Pending<Infallible>>);

impl StopToken {
    /// "Stops" the flag associated with this token.
    ///
    /// Note that calling this function multiple times does nothing, only the
    /// first call changes the state.
    pub fn stop(&self) {
        self.0.abort()
    }
}

impl StopFlag {
    /// Returns true if the stop token linked to `self` was used.
    #[must_use]
    pub fn is_stopped(&self) -> bool {
        self.0.is_aborted()
    }
}

/// This future resolves when a stop token was used.
impl Future for StopFlag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        self.project().0.poll(cx).map(|_res| ())
    }
}
