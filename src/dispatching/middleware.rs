use std::{future::Future, pin::Pin};

/// An asynchronous middleware.
///
/// See [the module-level documentation for the design
/// overview](crate::dispatching).
pub trait Middleware<T> {
    #[must_use]
    fn handle<'a>(&'a self, val: T) -> Pin<Box<dyn Future<Output = T> + 'a>>
    where
        T: 'a;
}

impl<T, F, Fut> Middleware<T> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = T>,
{
    fn handle<'a>(&'a self, val: T) -> Pin<Box<dyn Future<Output = T> + 'a>>
    where
        T: 'a,
    {
        Box::pin(async move { self(val).await })
    }
}
