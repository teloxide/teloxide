/// A result of a handler in [`Dispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DispatcherHandlerResult<Upd, E> {
    pub next: Option<Upd>,
    pub result: Result<(), E>,
}

impl<Upd, E> DispatcherHandlerResult<Upd, E> {
    /// Creates new `DispatcherHandlerResult` that continues the pipeline.
    pub fn next(update: Upd, result: Result<(), E>) -> Self {
        Self {
            next: Some(update),
            result,
        }
    }

    /// Creates new `DispatcherHandlerResult` that terminates the pipeline.
    pub fn exit(result: Result<(), E>) -> Self {
        Self { next: None, result }
    }
}

impl<Upd, E> From<Result<(), E>> for DispatcherHandlerResult<Upd, E> {
    fn from(result: Result<(), E>) -> Self {
        Self::exit(result)
    }
}
