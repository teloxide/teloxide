use futures::future::BoxFuture;
use std::{convert::Infallible, fmt::Debug, future::Future, sync::Arc};

/// An asynchronous handler of an error.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait ErrorHandler<E> {
    #[must_use]
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()>;
}

impl<E, F, Fut> ErrorHandler<E> for F
where
    F: Fn(E) -> Fut + Send + Sync + 'static,
    E: Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        Box::pin(async move { self(error).await })
    }
}

/// A handler that silently ignores all errors.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::{ErrorHandler, IgnoringErrorHandler};
///
/// IgnoringErrorHandler::new().handle_error(()).await;
/// IgnoringErrorHandler::new().handle_error(404).await;
/// IgnoringErrorHandler::new().handle_error("error").await;
/// # }
/// ```
pub struct IgnoringErrorHandler;

impl IgnoringErrorHandler {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

impl<E> ErrorHandler<E> for IgnoringErrorHandler {
    fn handle_error(self: Arc<Self>, _: E) -> BoxFuture<'static, ()> {
        Box::pin(async {})
    }
}

/// A handler that silently ignores all errors that can never happen (e.g.:
/// [`!`] or [`Infallible`]).
///
/// ## Examples
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use std::convert::{Infallible, TryInto};
///
/// use teloxide::dispatching::{ErrorHandler, IgnoringErrorHandlerSafe};
///
/// let result: Result<String, Infallible> = "str".try_into();
/// match result {
///     Ok(string) => println!("{}", string),
///     Err(inf) => IgnoringErrorHandlerSafe::new().handle_error(inf).await,
/// }
///
/// IgnoringErrorHandlerSafe::new().handle_error(return).await; // return type of `return` is `!` (aka never)
/// # }
/// ```
///
/// ```compile_fail
/// use teloxide::dispatching::{ErrorHandler, IgnoringErrorHandlerSafe};
///
/// IgnoringErrorHandlerSafe.handle_error(0);
/// ```
///
/// [`!`]: https://doc.rust-lang.org/std/primitive.never.html
/// [`Infallible`]: std::convert::Infallible
pub struct IgnoringErrorHandlerSafe;

impl IgnoringErrorHandlerSafe {
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self)
    }
}

#[allow(unreachable_code)]
impl ErrorHandler<Infallible> for IgnoringErrorHandlerSafe {
    fn handle_error(self: Arc<Self>, _: Infallible) -> BoxFuture<'static, ()> {
        Box::pin(async {})
    }
}

/// A handler that log all errors passed into it.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::{ErrorHandler, LoggingErrorHandler};
///
/// LoggingErrorHandler::empty().handle_error(()).await;
/// LoggingErrorHandler::new("error").handle_error(404).await;
/// LoggingErrorHandler::new("error")
///     .handle_error("Invalid data type!")
///     .await;
/// # }
/// ```
pub struct LoggingErrorHandler {
    text: String,
}

impl LoggingErrorHandler {
    /// Creates `LoggingErrorHandler` with a meta text before a log.
    ///
    /// The logs will be printed in this format: `{text}: {:?}`.
    #[must_use]
    pub fn new<T>(text: T) -> Arc<Self>
    where
        T: Into<String>,
    {
        Arc::new(Self { text: text.into() })
    }

    /// A shortcut for `LoggingErrorHandler::new("Error".to_owned())`.
    #[must_use]
    pub fn empty() -> Arc<Self> {
        Self::new("Error".to_owned())
    }
}

impl<E> ErrorHandler<E> for LoggingErrorHandler
where
    E: Debug,
{
    fn handle_error(self: Arc<Self>, error: E) -> BoxFuture<'static, ()> {
        log::error!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}
