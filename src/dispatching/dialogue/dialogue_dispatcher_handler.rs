use crate::prelude::{DialogueStage, DialogueWithCx};
use futures::future::BoxFuture;
use std::{future::Future, sync::Arc};

/// An asynchronous handler of an update used in [`DialogueDispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching::dialogue).
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub trait DialogueDispatcherHandler<R, Upd, D, E> {
    #[must_use]
    fn handle(
        self: Arc<Self>,
        cx: DialogueWithCx<R, Upd, D, E>,
    ) -> BoxFuture<'static, DialogueStage<D>>
    where
        DialogueWithCx<R, Upd, D, E>: Send + 'static,
        R: Send,
        Upd: Send,
        D: Send,
        E: Send;
}

impl<R, Upd, D, E, F, Fut> DialogueDispatcherHandler<R, Upd, D, E> for F
where
    F: Fn(DialogueWithCx<R, Upd, D, E>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = DialogueStage<D>> + Send + 'static,
{
    fn handle(self: Arc<Self>, cx: DialogueWithCx<R, Upd, D, E>) -> BoxFuture<'static, Fut::Output>
    where
        DialogueWithCx<R, Upd, D, E>: Send + 'static,
        R: Send,
        Upd: Send,
        D: Send,
        E: Send,
    {
        Box::pin(async move { self(cx).await })
    }
}
