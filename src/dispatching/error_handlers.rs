//! Error handlers.

// Infallible used here instead of `!` to be compatible with rust <1.41.
use std::{convert::Infallible, fmt::Debug, future::Future, pin::Pin};

/// An asynchronous handler of an error.
pub trait ErrorHandler<E> {
    #[must_use]
    fn handle_error<'a>(
        &'a self,
        error: E,
    ) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a;
}

/// A handler that silently ignores all errors.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::error_handlers::{ErrorHandler, Ignore};
///
/// Ignore.handle_error(()).await;
/// Ignore.handle_error(404).await;
/// Ignore.handle_error(String::from("error")).await;
/// # }
/// ```
pub struct Ignore;

impl<E> ErrorHandler<E> for Ignore {
    #[must_use]
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

/// An error handler that silently ignores all errors that can never happen
/// (e.g.: [`!`] or [`Infallible`]).
///
/// ## Examples
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use std::convert::{Infallible, TryInto};
///
/// use teloxide::dispatching::error_handlers::{ErrorHandler, IgnoreSafe};
///
/// let result: Result<String, Infallible> = "str".try_into();
/// match result {
///     Ok(string) => println!("{}", string),
///     Err(inf) => IgnoreSafe.handle_error(inf).await,
/// }
///
/// IgnoreSafe.handle_error(return).await; // return type of `return` is `!` (aka never)
/// # }
/// ```
///
/// ```compile_fail
/// use teloxide::dispatching::dispatchers::filter::error_policy::{
///     ErrorPolicy, IgnoreSafe,
/// };
///
/// IgnoreSafe.handle_error(0);
/// ```
///
/// [`!`]: https://doc.rust-lang.org/std/primitive.never.html
/// [`Infallible`]: std::convert::Infallible
pub struct IgnoreSafe;

#[allow(unreachable_code)]
impl ErrorHandler<Infallible> for IgnoreSafe {
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

/// An error handler that prints all errors passed into it.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::error_handlers::{ErrorHandler, Print};
///
/// Print.handle_error(()).await;
/// Print.handle_error(404).await;
/// Print.handle_error(String::from("error")).await;
/// # }
/// ```
pub struct Print;

impl<E> ErrorHandler<E> for Print
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
        log::debug!("error: {:?}", error);
        Box::pin(async {})
    }
}

/// The implementation of `ErrorHandler` for `Fn(error) -> Future<Output = ()>`.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::error_handlers::ErrorHandler;
///
/// let mut closure = |e: i32| async move { eprintln!("Error code{}", e) };
///
/// closure.handle_error(404).await;
/// # }
/// ```
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
