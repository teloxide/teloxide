use std::pin::Pin;

use crate::prelude::{DialogueDispatcherHandlerCtx, DialogueStage};
use std::future::Future;

/// An asynchronous handler of an update used in [`DialogueDispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub trait DialogueDispatcherHandler<Upd, D> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        ctx: DialogueDispatcherHandlerCtx<Upd, D>,
    ) -> Pin<Box<dyn Future<Output = DialogueStage<D>> + Send + Sync + 'a>>
    where
        DialogueDispatcherHandlerCtx<Upd, D>: Send + Sync + 'a;
}

impl<Upd, D, F, Fut> DialogueDispatcherHandler<Upd, D> for F
where
    F: Fn(DialogueDispatcherHandlerCtx<Upd, D>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + Sync + 'static,
{
    fn handle<'a>(
        &'a self,
        ctx: DialogueDispatcherHandlerCtx<Upd, D>,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + Send + Sync + 'a>>
    where
        DialogueDispatcherHandlerCtx<Upd, D>: Send + Sync + 'a,
    {
        Box::pin(async move { self(ctx).await })
    }
}
