use crate::types::Update;
use std::{future::Future, pin::Pin};

#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum SessionState<S> {
    Continue(S),
    Terminate,
}

pub trait Handler<S> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        session: S,
        update: Update,
    ) -> Pin<Box<dyn Future<Output = SessionState<S>> + 'a>>
    where
        S: 'a;
}

/// The implementation of `Handler` for `Fn(S, Update) -> Future<Output =
/// SessionState<S>>`.
impl<S, F, Fut> Handler<S> for F
where
    F: Fn(S, Update) -> Fut,
    Fut: Future<Output = SessionState<S>>,
{
    fn handle<'a>(
        &'a self,
        session: S,
        update: Update,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        S: 'a,
    {
        Box::pin(async move { self(session, update).await })
    }
}
