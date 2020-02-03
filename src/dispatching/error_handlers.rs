use std::{convert::Infallible, fmt::Debug, future::Future, pin::Pin};

/// An asynchronous handler of an error.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait ErrorHandler<E> {
    #[must_use]
    fn handle_error<'a>(
        &'a self,
        error: E,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a;
}

impl<E, F, Fut> ErrorHandler<E> for F
where
    F: Fn(E) -> Fut,
    Fut: Future<Output = ()>,
{
    fn handle_error<'a>(
        &'a self,
        error: E,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a,
    {
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
/// IgnoringErrorHandler.handle_error(()).await;
/// IgnoringErrorHandler.handle_error(404).await;
/// IgnoringErrorHandler.handle_error("error").await;
/// # }
/// ```
pub struct IgnoringErrorHandler;

impl<E> ErrorHandler<E> for IgnoringErrorHandler {
    fn handle_error<'a>(
        &'a self,
        _: E,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a,
    {
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
///     Err(inf) => IgnoringErrorHandlerSafe.handle_error(inf).await,
/// }
///
/// IgnoringErrorHandlerSafe.handle_error(return).await; // return type of `return` is `!` (aka never)
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

#[allow(unreachable_code)]
impl ErrorHandler<Infallible> for IgnoringErrorHandlerSafe {
    fn handle_error<'a>(
        &'a self,
        _: Infallible,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        Infallible: 'a,
    {
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
/// LoggingErrorHandler::default().handle_error(()).await;
/// LoggingErrorHandler::new("error").handle_error(404).await;
/// LoggingErrorHandler::new("error")
///     .handle_error("Invalid data type!")
///     .await;
/// # }
/// ```
#[derive(Default)]
pub struct LoggingErrorHandler {
    text: String,
}

impl LoggingErrorHandler {
    /// Creates `LoggingErrorHandler` with a meta text before a log.
    ///
    /// The logs will be printed in this format: `{text}: {:?}`.
    #[must_use]
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self { text: text.into() }
    }
}

impl<E> ErrorHandler<E> for LoggingErrorHandler
where
    E: Debug,
{
    fn handle_error<'a>(
        &'a self,
        error: E,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a,
    {
        log::debug!("{text}: {:?}", error, text = self.text);
        Box::pin(async {})
    }
}
