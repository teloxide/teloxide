use crate::dispatching::core::{Guard, IntoGuard, IntoHandler, Handler};

pub trait HandlerBuilderWithGuards<Ctx, Err> {
    fn with_guard<G: Guard<Ctx> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<Ctx, G> + 'static,
    ) -> Self;

    fn or_with_guard<G: Guard<Ctx> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<Ctx, G> + 'static,
    ) -> Self;

    fn or_else<F, H>(self, func: F) -> Self
    where
        F: IntoHandler<H>,
        H: Handler<Ctx, Err> + Send + Sync + 'static;
}