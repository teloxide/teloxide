/// A result of a handler in [`Dispatcher`].
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct DispatcherHandlerResult<Upd, E> {
    next: Option<Upd>,
    result: Result<(), E>,
}

impl<Upd, E> DispatcherHandlerResult<Upd, E> {
    /// Creates new `DispatcherHandlerResult`.
    pub fn new(next: Option<Upd>, result: Result<(), E>) -> Self {
        Self { next, result }
    }
}

impl<Upd, E> From<Result<(), E>> for DispatcherHandlerResult<Upd, E> {
    fn from(result: Result<(), E>) -> Self {
        Self::new(None, result)
    }
}
