use std::{future::Future, pin::Pin};

/// An asynchronous handler of a value.
pub trait Handler<T, E> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        value: T,
    ) -> Pin<Box<dyn Future<Output = Result<(), E>> + 'a>>
    where
        T: 'a;
}

/// The implementation of `Handler` for `Fn(U) -> Future<Output = Result<(),
/// E>`.
///
/// Looks quite strange for now, but with stabilised asynchronous traits it
/// should be prettier.
impl<T, E, F, Fut> Handler<T, E> for F
where
    F: Fn(T) -> Fut,
    Fut: Future<Output = Result<(), E>>,
{
    fn handle<'a>(
        &'a self,
        value: T,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        T: 'a,
    {
        Box::pin(async move { self(value).await })
    }
}
