use std::future::Future;

use async_trait::async_trait;

/// Implementors of this trait are treated as error-handlers.
#[async_trait]
pub trait ErrorPolicy<E> {
    async fn handle_error(&mut self, error: E);
}

/// A convenient structure with an error-handling closure. Implements
/// `ErrorPolicy`.
pub struct FnErrorPolicy<F>(pub F);

#[async_trait]
impl<E, F, Fut> ErrorPolicy<E> for FnErrorPolicy<F>
where
    F: FnMut(E) -> Fut + Send,
    Fut: Future<Output = ()>,
    E: Send,
{
    async fn handle_error(&mut self, error: E) {
        self.0(error);
    }
}
