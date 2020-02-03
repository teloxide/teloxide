use std::{future::Future, pin::Pin};

/// An asynchronous handler of a context.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait CtxHandler<Ctx, Output> {
    #[must_use]
    fn handle_ctx<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Output> + 'a>>
    where
        Ctx: 'a;
}

impl<Ctx, Output, F, Fut> CtxHandler<Ctx, Output> for F
where
    F: Fn(Ctx) -> Fut,
    Fut: Future<Output = Output>,
{
    fn handle_ctx<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        Ctx: 'a,
    {
        Box::pin(async move { self(ctx).await })
    }
}
