use std::{future::Future, pin::Pin, task};

use futures::future::{pending, AbortHandle, Abortable, Pending};

/// A stop token allows you to stop listener.
///
/// See also: [`UpdateListener::stop_token`].
///
/// [`UpdateListener::stop_token`]:
/// crate::dispatching::update_listeners::UpdateListener::stop_token
pub trait StopToken {
    /// Stop the listener linked to this token.
    fn stop(self);
}

/// A stop token which does nothing. May be used in prototyping or in cases
/// where you do not care about graceful shutdowning.
pub struct Noop;

impl StopToken for Noop {
    fn stop(self) {}
}

/// A stop token which corresponds to [`AsyncStopFlag`].
#[derive(Clone)]
pub struct AsyncStopToken(AbortHandle);

/// A flag which corresponds to [`AsyncStopToken`].
///
/// To know if stop token was used you can either repeatedly call [`is_stopped`]
/// or use this type as a `Future`.
///
/// [`is_stopped`]: AsyncStopFlag::is_stopped
#[pin_project::pin_project]
pub struct AsyncStopFlag(#[pin] Abortable<Pending<()>>);

impl AsyncStopToken {
    /// Create a new token/flag pair.
    pub fn new_pair() -> (Self, AsyncStopFlag) {
        let (handle, reg) = AbortHandle::new_pair();
        let token = Self(handle);
        let flag = AsyncStopFlag(Abortable::new(pending(), reg));

        (token, flag)
    }
}

impl StopToken for AsyncStopToken {
    fn stop(self) {
        self.0.abort()
    }
}

impl AsyncStopFlag {
    /// Returns true if stop token linked to `self` was used.
    pub fn is_stopped(&self) -> bool {
        self.0.is_aborted()
    }
}

/// This future resolves when a stop token was used.
impl Future for AsyncStopFlag {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> task::Poll<Self::Output> {
        self.project().0.poll(cx).map(|res| {
            debug_assert!(
                res.is_err(),
                "Pending Future can't ever be resolved, so Abortable is only resolved when \
                 canceled"
            );
        })
    }
}
