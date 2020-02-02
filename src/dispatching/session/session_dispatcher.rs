use crate::dispatching::{
    session::{
        GetChatId, InMemStorage, SessionHandlerCtx, SessionState, Storage,
    },
    AsyncHandler, HandlerCtx,
};
use std::{future::Future, pin::Pin};

/// A dispatcher of user sessions.
///
/// Note that `SessionDispatcher` implements `AsyncHandler`, so you can just put
/// an instance of this dispatcher into the [`Dispatcher`]'s methods.
///
/// [`Dispatcher`]: crate::dispatching::Dispatcher
pub struct SessionDispatcher<'a, Session, H> {
    storage: Box<dyn Storage<Session> + 'a>,
    handler: H,
}

impl<'a, Session, H> SessionDispatcher<'a, Session, H>
where
    Session: Default + 'a,
{
    /// Creates a dispatcher with the specified `handler` and [`InMemStorage`]
    /// (a default storage).
    ///
    /// [`InMemStorage`]: crate::dispatching::session::InMemStorage
    #[must_use]
    pub fn new(handler: H) -> Self {
        Self {
            storage: Box::new(InMemStorage::default()),
            handler,
        }
    }

    /// Creates a dispatcher with the specified `handler` and `storage`.
    #[must_use]
    pub fn with_storage<Stg>(handler: H, storage: Stg) -> Self
    where
        Stg: Storage<Session> + 'a,
    {
        Self {
            storage: Box::new(storage),
            handler,
        }
    }
}

impl<'a, Session, H, Upd> AsyncHandler<HandlerCtx<Upd>, Result<(), ()>>
    for SessionDispatcher<'a, Session, H>
where
    H: AsyncHandler<SessionHandlerCtx<Upd, Session>, SessionState<Session>>,
    Upd: GetChatId,
    Session: Default,
{
    /// Dispatches a single `message` from a private chat.
    fn handle<'b>(
        &'b self,
        ctx: HandlerCtx<Upd>,
    ) -> Pin<Box<dyn Future<Output = Result<(), ()>> + 'b>>
    where
        Upd: 'b,
    {
        Box::pin(async move {
            let chat_id = ctx.update.chat_id();

            let session = self
                .storage
                .remove_session(chat_id)
                .await
                .unwrap_or_default();

            if let SessionState::Next(new_session) = self
                .handler
                .handle(SessionHandlerCtx {
                    bot: ctx.bot,
                    update: ctx.update,
                    session,
                })
                .await
            {
                if self
                    .storage
                    .update_session(chat_id, new_session)
                    .await
                    .is_some()
                {
                    panic!(
                        "We previously storage.remove_session() so \
                         storage.update_session() must return None"
                    );
                }
            }

            Ok(())
        })
    }
}
