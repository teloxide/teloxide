use std::pin::Pin;
use std::future::Future;


/// Asynchronous handler for event `I` (like `&self, I -> Future` fn)
pub trait Handler<I> {
    fn handle(&self, value: I) -> Pin<Box<dyn Future<Output=()> + 'static>>;
}

impl<Fut, T, F> Handler<T> for F
where
    Fut: Future<Output = ()> + 'static,
    F: Fn(T) -> Fut,
{
    fn handle(&self, value: T) -> Pin<Box<dyn Future<Output=()> + 'static>> {
        Box::pin((self)(value))
    }
}
