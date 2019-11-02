use std::future::Future;

use async_trait::async_trait;

/// Implementors of this trait are treated as error-handlers.
#[async_trait]
pub trait ErrorPolicy<E> {
    async fn handle_error(&self, error: E);
}

#[async_trait]
impl<E, F, Fut> ErrorPolicy<E> for F
where
    F: Fn(E) -> Fut + Sync,
    Fut: Future<Output = ()> + Send + 'static,
    E: Send + 'static,
{
    async fn handle_error(&self, error: E) {
        self(error).await;
    }
}
