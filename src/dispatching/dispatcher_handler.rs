use std::{future::Future, pin::Pin};

use crate::dispatching::{DispatcherHandlerCtx, DispatcherHandlerRx};

/// An asynchronous handler of a stream of updates used in [`Dispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub trait DispatcherHandler<Upd> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        updates: DispatcherHandlerRx<Upd>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>
    where
        DispatcherHandlerCtx<Upd>: Send + Sync + 'a;
}

impl<Upd, F, Fut> DispatcherHandler<Upd> for F
where
    F: Fn(DispatcherHandlerRx<Upd>) -> Fut + Send + Sync + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
    fn handle<'a>(
        &'a self,
        updates: DispatcherHandlerRx<Upd>,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'a>>
    where
        DispatcherHandlerCtx<Upd>: Send + Sync + 'a,
    {
        Box::pin(async move { self(updates).await })
    }
}
