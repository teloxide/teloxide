use std::future::Future;

use crate::dispatching::{DispatcherHandlerRx, UpdateWithCx};
use futures::future::BoxFuture;

/// An asynchronous handler of a stream of updates used in [`Dispatcher`].
///
/// See the [module-level documentation](crate::dispatching) for the design
/// overview.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
#[deprecated(note = "Use dispatching2 instead")]
pub trait DispatcherHandler<R, Upd> {
    #[must_use]
    fn handle(self, updates: DispatcherHandlerRx<R, Upd>) -> BoxFuture<'static, ()>
    where
        UpdateWithCx<R, Upd>: Send + 'static;
}

impl<R, Upd, F, Fut> DispatcherHandler<R, Upd> for F
where
    F: FnOnce(DispatcherHandlerRx<R, Upd>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    fn handle(self, updates: DispatcherHandlerRx<R, Upd>) -> BoxFuture<'static, ()>
    where
        UpdateWithCx<R, Upd>: Send + 'static,
    {
        Box::pin(async move { self(updates).await })
    }
}
