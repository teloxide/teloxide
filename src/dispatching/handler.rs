use std::{future::Future, pin::Pin};

/// Continue or terminate a user session.
#[derive(Debug, Copy, Clone, Eq, Hash, PartialEq)]
pub enum SessionState<Session> {
    Continue(Session),
    Terminate,
}

/// A handler of a user session and an update.
///
/// ## Returns
/// Returns [`SessionState::Continue(session)`] if it wants to be called again
/// after a new update, or [`SessionState::Terminate`] if not.
///
/// [`SessionState::Continue(session)`]:
/// crate::dispatching::SessionState::Continue
/// [`SessionState::Terminate`]:  crate::dispatching::SessionState::Terminate
pub trait Handler<Session, U> {
    #[must_use]
    fn handle<'a>(
        &'a self,
        session: Session,
        update: U,
    ) -> Pin<Box<dyn Future<Output = SessionState<Session>> + 'a>>
    where
        Session: 'a,
        U: 'a;
}

/// The implementation of `Handler` for `Fn(Session, Update) -> Future<Output =
/// SessionState<Session>>`.
impl<Session, U, F, Fut> Handler<Session, U> for F
where
    F: Fn(Session, U) -> Fut,
    Fut: Future<Output = SessionState<Session>>,
{
    fn handle<'a>(
        &'a self,
        session: Session,
        update: U,
    ) -> Pin<Box<dyn Future<Output = Fut::Output> + 'a>>
    where
        Session: 'a,
        U: 'a,
    {
        Box::pin(async move { self(session, update).await })
    }
}
