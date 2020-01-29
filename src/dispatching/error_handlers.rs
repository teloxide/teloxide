//! Commonly used handlers of errors.

use crate::dispatching::AsyncHandler;
use std::{convert::Infallible, fmt::Debug, future::Future, pin::Pin};

/// A handler that silently ignores all errors.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::{error_handlers::Ignore, AsyncHandler};
///
/// Ignore.handle(()).await;
/// Ignore.handle(404).await;
/// Ignore.handle(String::from("error")).await;
/// # }
/// ```
pub struct Ignore;

impl<E> AsyncHandler<E, ()> for Ignore {
    fn handle<'a>(&'a self, _: E) -> Pin<Box<dyn Future<Output = ()> + 'a>>
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
/// use teloxide::dispatching::{AsyncHandler, error_handlers::IgnoreSafe};
///
/// let result: Result<String, Infallible> = "str".try_into();
/// match result {
///     Ok(string) => println!("{}", string),
///     Err(inf) => IgnoreSafe.handle(inf).await,
/// }
///
/// IgnoreSafe.handle(return).await; // return type of `return` is `!` (aka never)
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
impl AsyncHandler<Infallible, ()> for IgnoreSafe {
    fn handle<'a>(
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
/// use teloxide::dispatching::{error_handlers::Log, AsyncHandler};
///
/// Log.handle(()).await;
/// Log.handle(404).await;
/// Log.handle(String::from("error")).await;
/// # }
/// ```
pub struct Log;

impl<E> AsyncHandler<E, ()> for Log
where
    E: Debug,
{
    fn handle<'a>(&'a self, error: E) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        E: 'a,
    {
        log::debug!("error: {:?}", error);
        Box::pin(async {})
    }
}
