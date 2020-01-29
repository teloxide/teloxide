use std::{future::Future, pin::Pin};

/// An asynchronous polymorphic handler of a context.
pub trait Handler<Ctx, Output> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Output> + 'a>>
    where
        Ctx: 'a;
}

/// The implementation of `Handler` for `Fn(Ctx) -> Future<Output = Output>`.
impl<Ctx, Output, F, Fut> Handler<Ctx, Output> for F
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
