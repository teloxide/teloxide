use std::{convert::Infallible, fmt::Debug, future::Future, pin::Pin};

/// An asynchronous polymorphic handler of a context.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait AsyncHandler<Ctx, Output> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Output> + 'a>>
    where
        Ctx: 'a;
}

impl<Ctx, Output, F, Fut> AsyncHandler<Ctx, Output> for F
where
    F: Fn(Ctx) -> Fut,
    Fut: Future<Output = Output>,
{
    fn handle<'a>(
        &'a self,
        ctx: Ctx,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        Ctx: 'a,
    {
        Box::pin(async move { self(ctx).await })
    }
}

/// A handler that silently ignores all values.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::{AsyncHandler, IgnoringHandler};
///
/// IgnoringHandler.handle(()).await;
/// IgnoringHandler.handle(404).await;
/// IgnoringHandler.handle("error").await;
/// # }
/// ```
pub struct IgnoringHandler;

impl<Ctx> AsyncHandler<Ctx, ()> for IgnoringHandler {
    fn handle<'a>(&'a self, _: Ctx) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        Ctx: 'a,
    {
        Box::pin(async {})
    }
}

/// A handler that silently ignores all values that can never happen (e.g.:
/// [`!`] or [`Infallible`]).
///
/// ## Examples
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use std::convert::{Infallible, TryInto};
///
/// use teloxide::dispatching::{AsyncHandler, IgnoringHandlerSafe};
///
/// let result: Result<String, Infallible> = "str".try_into();
/// match result {
///     Ok(string) => println!("{}", string),
///     Err(inf) => IgnoringHandlerSafe.handle(inf).await,
/// }
///
/// IgnoringHandlerSafe.handle(return).await; // return type of `return` is `!` (aka never)
/// # }
/// ```
///
/// ```compile_fail
/// use teloxide::dispatching::{AsyncHandler, IgnoringHandlerSafe};
///
/// IgnoringHandlerSafe.handle(0);
/// ```
///
/// [`!`]: https://doc.rust-lang.org/std/primitive.never.html
/// [`Infallible`]: std::convert::Infallible
pub struct IgnoringHandlerSafe;

#[allow(unreachable_code)]
impl AsyncHandler<Infallible, ()> for IgnoringHandlerSafe {
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

/// A handler that log all values passed into it.
///
/// ## Example
/// ```
/// # #[tokio::main]
/// # async fn main_() {
/// use teloxide::dispatching::{AsyncHandler, LoggingHandler};
///
/// LoggingHandler::default().handle(()).await;
/// LoggingHandler::new("error").handle(404).await;
/// LoggingHandler::new("error")
///     .handle("Invalid data type!")
///     .await;
/// # }
/// ```
#[derive(Default)]
pub struct LoggingHandler {
    text: String,
}

impl LoggingHandler {
    /// Creates `LoggingHandler` with a meta text before a log.
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

impl<Ctx> AsyncHandler<Ctx, ()> for LoggingHandler
where
    Ctx: Debug,
{
    fn handle<'a>(&'a self, ctx: Ctx) -> Pin<Box<dyn Future<Output = ()> + 'a>>
    where
        Ctx: 'a,
    {
        log::debug!("{text}: {:?}", ctx, text = self.text);
        Box::pin(async {})
    }
}
