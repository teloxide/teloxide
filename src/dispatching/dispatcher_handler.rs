use std::future::Future;

use crate::dispatching::{DispatcherHandlerRx, UpdateWithCx};
use futures::future::BoxFuture;

/// An asynchronous handler of a stream of updates used in [`Dispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub trait DispatcherHandler<Upd> {
    #[must_use]
    fn handle(
        self,
        updates: DispatcherHandlerRx<Upd>,
    ) -> BoxFuture<'static, ()>
    where
        UpdateWithCx<Upd>: Send + 'static;
}

impl<Upd, F, Fut> DispatcherHandler<Upd> for F
where
    F: FnOnce(DispatcherHandlerRx<Upd>) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    fn handle(self, updates: DispatcherHandlerRx<Upd>) -> BoxFuture<'static, ()>
    where
        UpdateWithCx<Upd>: Send + 'static,
    {
        Box::pin(async move { self(updates).await })
    }
}
