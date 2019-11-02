use std::future::Future;

use async_trait::async_trait;
use std::pin::Pin;

/// Implementors of this trait are treated as error-handlers.
#[async_trait]
pub trait ErrorPolicy<E> {
    async fn handle_error(&self, error: E)
    where
        E: 'async_trait;
}

impl<E, F, Fut> ErrorPolicy<E> for F
    where
        F: Fn(E) -> Fut + Sync,
        Fut: Future<Output = ()> + Send,
        E: Send,
{
    fn handle_error<'s, 'async_trait>(&'s self, error: E) -> Pin<Box<dyn Future<Output = ()> + Send + 'async_trait>>
    where
        's: 'async_trait,
        Self: 'async_trait,
        E: 'async_trait
    {
        Box::pin(async move { self(error).await })
    }
}
