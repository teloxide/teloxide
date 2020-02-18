use crate::prelude::{DialogueDispatcherHandlerCx, DialogueStage};
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// An asynchronous handler of an update used in [`DialogueDispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub trait DialogueDispatcherHandler<Upd, D> {
    #[must_use]
    fn handle(
        self: Arc<Self>,
        cx: DialogueDispatcherHandlerCx<Upd, D>,
    ) -> BoxFuture<'static, DialogueStage<D>>
    where
        DialogueDispatcherHandlerCx<Upd, D>: Send + 'static;
}

impl<Upd, D, F, Fut> DialogueDispatcherHandler<Upd, D> for F
where
    F: Fn(DialogueDispatcherHandlerCx<Upd, D>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
{
    fn handle(
        self: Arc<Self>,
        cx: DialogueDispatcherHandlerCx<Upd, D>,
    ) -> BoxFuture<'static, Fut::Output>
    where
        DialogueDispatcherHandlerCx<Upd, D>: Send + 'static,
    {
        Box::pin(async move { self(cx).await })
    }
}
