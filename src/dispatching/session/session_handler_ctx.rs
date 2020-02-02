use crate::{
    dispatching::session::GetChatId,
    requests::{Request, ResponseResult},
    types::Message,
    Bot,
};
use std::sync::Arc;

/// A context of a [`SessionDispatcher`]'s message handler.
///
/// [`SessionDispatcher`]: crate::dispatching::session::SessionDispatcher
pub struct SessionHandlerCtx<Upd, Session> {
    pub bot: Arc<Bot>,
    pub update: Upd,
    pub session: Session,
}

impl<Upd, Session> GetChatId for SessionHandlerCtx<Upd, Session>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl<Session> SessionHandlerCtx<Message, Session> {
    pub async fn reply<T>(&self, text: T) -> ResponseResult<()>
    where
        T: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .send()
            .await
            .map(|_| ())
    }
}
