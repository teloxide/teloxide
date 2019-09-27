use std::future::Future;
use std::pin::Pin;

/// Asynchronous handler for event `I` (like `&self, I -> Future` fn)
pub trait Handler<'a, I> {
    fn handle(&self, value: I) -> Pin<Box<dyn Future<Output = ()> + 'a>>;
}

impl<'a, Fut, T, F> Handler<'a, T> for F
where
    Fut: Future<Output = ()> + 'a,
    F: Fn(T) -> Fut,
{
    fn handle(&self, value: T) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
        Box::pin((self)(value))
    }
}
