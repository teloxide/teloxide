use std::{convert::Infallible, future::Future, pin::Pin};

use async_trait::async_trait;

/// Implementors of this trait are treated as error-handlers.
#[async_trait]
pub trait ErrorPolicy<E> {
    async fn handle_error(&self, error: E)
    where
        E: 'async_trait;
}

/// Error policy that silently ignores all errors
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use telebofr::dispatching::dispatchers::filter::error_policy::{
///     ErrorPolicy, Ignore,
/// };
///
/// Ignore.handle_error(()).await;
/// Ignore.handle_error(404).await;
/// Ignore.handle_error(String::from("error")).await;
/// # }
/// ```
pub struct Ignore;

#[async_trait]
impl<E> ErrorPolicy<E> for Ignore
where
    E: Send,
{
    async fn handle_error(&self, _: E)
    where
        E: 'async_trait,
    {
    }
}

/// Error policy that silently ignores all errors that can never happen (e.g.:
/// [`!`] or [`Infallible`])
///
/// ## Examples
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use std::convert::{TryInto, Infallible};
///
/// use telebofr::dispatching::dispatchers::filter::error_policy::{
///     ErrorPolicy,
///     IgnoreSafe,
/// };
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
/// use telebofr::dispatching::dispatchers::filter::error_policy::{
///     ErrorPolicy, IgnoreSafe,
/// };
///
/// IgnoreSafe.handle_error(0);
/// ```
///
/// ## Note
/// Never type is not stabilized yet (see [`#35121`]) so all API that uses [`!`]
/// (including `impl ErrorPolicy<!> for IgnoreSafe`) we hide under the
/// `never-type` cargo feature.
///
/// [`!`]: https://doc.rust-lang.org/std/primitive.never.html
/// [`Infallible`]: std::convert::Infallible
/// [`#35121`]: https://github.com/rust-lang/rust/issues/35121
pub struct IgnoreSafe;

#[cfg(feature = "never-type")]
#[allow(unreachable_code)]
#[async_trait]
impl ErrorPolicy<!> for IgnoreSafe {
    async fn handle_error(&self, _: !)
    where
        !: 'async_trait,
    {
        never
    }
}

#[allow(unreachable_code)]
#[async_trait]
impl ErrorPolicy<Infallible> for IgnoreSafe {
    async fn handle_error(&self, _: Infallible)
    where
        Infallible: 'async_trait,
    {}
}

/// Implementation of `ErrorPolicy` for `async fn`s
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// use telebofr::dispatching::dispatchers::filter::error_policy::ErrorPolicy;
///
/// let closure = |e: i32| async move { eprintln!("Error code{}", e) };
///
/// closure.handle_error(404).await;
/// # }
/// ```
impl<E, F, Fut> ErrorPolicy<E> for F
where
    F: Fn(E) -> Fut + Sync,
    Fut: Future<Output = ()> + Send,
    E: Send,
{
    fn handle_error<'s, 'async_trait>(
        &'s self,
        error: E,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        's: 'async_trait,
        Self: 'async_trait,
        E: 'async_trait,
    {
        Box::pin(async move { self(error).await })
    }
}
