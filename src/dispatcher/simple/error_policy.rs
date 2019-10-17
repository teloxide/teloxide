use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

// TODO: shouldn't it be trait?
pub enum ErrorPolicy<'a, E> {
    Ignore,
    Log,
    Custom(Box<dyn Fn(E) -> Pin<Box<dyn Future<Output = ()> + 'a>>>),
}

impl<'a, E> ErrorPolicy<'a, E>
where
    E: Debug,
{
    pub async fn handle_error(&self, error: E) {
        match self {
            Self::Ignore => {}
            Self::Log => {
                // TODO: better message
                log::error!("Error in handler: {:?}", error)
            }
            Self::Custom(func) => func(error).await,
        }
    }

    pub fn custom<F, Fut>(f: F) -> Self
    where
        F: Fn(E) -> Fut + 'static,
        Fut: Future<Output = ()> + 'a,
    {
        Self::Custom(Box::new(move |e| Box::pin(f(e))))
    }
}
