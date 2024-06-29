//! Convenient error handling.

use std::{convert::Infallible, fmt::Debug, future::Future, sync::Arc};

use dptree::di::DependencyMap;
use futures::future::BoxFuture;

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

/// An asynchronous handler of an error with dependencies
pub trait ErrorHandlerExt<E> {
    #[must_use]
    fn handle_error_with_deps(
        self: Arc<Self>,
        deps: DependencyMap,
        error: E,
    ) -> BoxFuture<'static, ()>;
}

impl<E, F, Fut> ErrorHandlerExt<E> for F
where
    F: Fn(DependencyMap, E) -> Fut + Send + Sync + 'static,
    E: Send + 'static,
    Fut: Future<Output = ()> + Send,
{
    fn handle_error_with_deps(
        self: Arc<Self>,
        deps: DependencyMap,
        error: E,
    ) -> BoxFuture<'static, ()> {
        Box::pin(async move { self(deps, error).await })
    }
}

/// Something that can be handled by an error handler.
///
/// ## Examples
/// ```
/// use teloxide::error_handlers::OnError;
///
/// # #[tokio::main]
/// # async fn main_() {
/// // Prints nothing
/// let ok: Result<i32, i32> = Ok(200);
/// ok.log_on_error().await;
///
/// // Prints "Error: 404"
/// let err: Result<i32, i32> = Err(404);
/// err.log_on_error().await;
/// # }
/// ```
///
/// Use an arbitrary error handler:
/// ```
/// use teloxide::error_handlers::{IgnoringErrorHandler, OnError};
///
/// # #[tokio::main]
/// # async fn main_() {
/// let err: Result<i32, i32> = Err(404);
/// err.on_error(IgnoringErrorHandler::new()).await;
/// # }
/// ```
pub trait OnError<E> {
    #[must_use]
    fn on_error<'a, Eh>(self, eh: Arc<Eh>) -> BoxFuture<'a, ()>
    where
        Self: 'a,
        Eh: ErrorHandler<E> + Send + Sync,
        Arc<Eh>: 'a;

    /// A shortcut for `.on_error(LoggingErrorHandler::new())`.
    #[must_use]
    fn log_on_error<'a>(self) -> BoxFuture<'a, ()>
    where
        Self: Sized + 'a,
        E: Debug,
    {
        self.on_error(LoggingErrorHandler::new())
    }
}

impl<T, E> OnError<E> for Result<T, E>
where
    T: Send,
    E: Send,
{
    fn on_error<'a, Eh>(self, eh: Arc<Eh>) -> BoxFuture<'a, ()>
    where
        Self: 'a,
        Eh: ErrorHandler<E> + Send + Sync,
        Arc<Eh>: 'a,
    {
        Box::pin(async move {
            if let Err(error) = self {
                eh.handle_error(error).await;
            }
        })
    }
}

/// A handler that silently ignores all errors.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::error_handlers::{ErrorHandler, IgnoringErrorHandler};
///
/// IgnoringErrorHandler::new().handle_error(()).await;
/// IgnoringErrorHandler::new().handle_error(404).await;
/// IgnoringErrorHandler::new().handle_error("error").await;
/// # }
/// ```
#[derive(Clone, Copy)]
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

impl<E> ErrorHandlerExt<E> for IgnoringErrorHandler {
    fn handle_error_with_deps(
        self: Arc<Self>,
        _deps: DependencyMap,
        _: E,
    ) -> BoxFuture<'static, ()> {
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
/// use teloxide::error_handlers::{ErrorHandler, IgnoringErrorHandlerSafe};
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
#[derive(Clone, Copy)]
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

#[allow(unreachable_code)]
impl ErrorHandlerExt<Infallible> for IgnoringErrorHandlerSafe {
    fn handle_error_with_deps(
        self: Arc<Self>,
        _deps: DependencyMap,
        _: Infallible,
    ) -> BoxFuture<'static, ()> {
        Box::pin(async {})
    }
}

/// A handler that log all errors passed into it.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::error_handlers::{ErrorHandler, LoggingErrorHandler};
///
/// LoggingErrorHandler::new().handle_error(()).await;
/// LoggingErrorHandler::with_custom_text("Omg1").handle_error(404).await;
/// LoggingErrorHandler::with_custom_text("Omg2").handle_error("Invalid data type!").await;
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
    pub fn with_custom_text<T>(text: T) -> Arc<Self>
    where
        T: Into<String>,
    {
        Arc::new(Self { text: text.into() })
    }

    /// A shortcut for
    /// `LoggingErrorHandler::with_custom_text("Error".to_owned())`.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Self::with_custom_text("Error".to_owned())
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

impl<E> ErrorHandlerExt<E> for LoggingErrorHandler
where
    E: Debug,
{
    fn handle_error_with_deps(
        self: Arc<Self>,
        _deps: DependencyMap,
        error: E,
    ) -> BoxFuture<'static, ()> {
        log::error!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}
