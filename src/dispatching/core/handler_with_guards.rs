use crate::dispatching::core::{Guard, Handler, IntoGuard, IntoHandler};

/// The trait is used for handler builders that can use [`Guard`]s.
///
/// [`Guard`]: crate::dispatching::dev::Guard
pub trait HandlerBuilderWithGuards<Ctx, Err> {
    /// The method add the specified guard to handler.
    fn with_guard<G: Guard<Ctx> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<Ctx, G> + 'static,
    ) -> Self;

    /// The method add the specified guard to the previous added guard using
    /// [`OrGuard`] or panics.
    ///
    /// [`OrGuard`]: crate::dispatching::dev::OrGuard
    fn or_with_guard<G: Guard<Ctx> + Send + Sync + 'static>(
        self,
        guard: impl IntoGuard<Ctx, G> + 'static,
    ) -> Self;

    /// The method add the specified handler which will be called if previously
    /// added guard return false.
    fn or_else<F, H>(self, func: F) -> Self
    where
        F: IntoHandler<H>,
        H: Handler<Ctx, Err> + Send + Sync + 'static;
}
