use std::{future::Future, pin::Pin};

/// An asynchronous polymorphic handler of a context.
///
/// Note that `AsyncHandler` is implemented for asynchronous `Fn`s, that consume
/// `Ctx` and return `Output`.
pub trait AsyncHandler<Ctx, Output> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Output> + 'a>>
    where
        Ctx: 'a;
}

impl<Ctx, Output, F, Fut> AsyncHandler<Ctx, Output> for F
where
    F: Fn(Ctx) -> Fut,
    Fut: Future<Output = Output>,
{
    fn handle<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        Ctx: 'a,
    {
        Box::pin(async move { self(ctx).await })
    }
}
