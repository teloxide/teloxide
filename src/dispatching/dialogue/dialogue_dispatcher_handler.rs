use crate::prelude::{DialogueDispatcherHandlerCx, DialogueStage};
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// An asynchronous handler of an update used in [`DialogueDispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub trait DialogueDispatcherHandler<Upd, D, E> {
    #[must_use]
    fn handle(
        self: Arc<Self>,
        cx: DialogueDispatcherHandlerCx<Upd, D, E>,
    ) -> BoxFuture<'static, DialogueStage<D>>
    where
        DialogueDispatcherHandlerCx<Upd, D, E>: Send + 'static;
}

impl<Upd, D, E, F, Fut> DialogueDispatcherHandler<Upd, D, E> for F
where
    F: Fn(DialogueDispatcherHandlerCx<Upd, D, E>) -> Fut
        + Send
        + Sync
        + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
{
    fn handle(
        self: Arc<Self>,
        cx: DialogueDispatcherHandlerCx<Upd, D, E>,
    ) -> BoxFuture<'static, Fut::Output>
    where
        DialogueDispatcherHandlerCx<Upd, D, E>: Send + 'static,
    {
        Box::pin(async move { self(cx).await })
    }
}