use futures::FutureExt;
use std::future::Future;
use std::pin::Pin;

pub type HandlerResult<E> = Result<(), E>;

/// Asynchronous handler for event `T` (like `&self, I -> Future` fn)
pub trait Handler<'a, T, E> {
    fn handle(&self, value: T) -> Pin<Box<dyn Future<Output = HandlerResult<E>> + 'a>>;
}

pub trait IntoHandlerResult<E> {
    fn into_hr(self) -> HandlerResult<E>;
}

impl<E> IntoHandlerResult<E> for () {
    fn into_hr(self) -> HandlerResult<E> {
        Ok(())
    }
}

impl<E> IntoHandlerResult<E> for HandlerResult<E> {
    fn into_hr(self) -> HandlerResult<E> {
        self
    }
}

impl<'a, F, Fut, R, T, E> Handler<'a, T, E> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = R> + 'a,
    R: IntoHandlerResult<E> + 'a,
    E: 'a,
{
    fn handle(&self, value: T) -> Pin<Box<dyn Future<Output = HandlerResult<E>> + 'a>> {
        Box::pin(self(value).map(IntoHandlerResult::into_hr))
    }
}
