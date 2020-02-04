use crate::{
    dispatching::dialogue::{Dialogue, GetChatId},
    requests::{Request, ResponseResult},
    types::Message,
    Bot,
};
use std::sync::Arc;

/// A context of a [`DialogueDispatcher`]'s message handler.
///
/// [`DialogueDispatcher`]: crate::dispatching::dialogue::DialogueDispatcher
pub struct DialogueHandlerCtx<Upd, State, T> {
    pub bot: Arc<Bot>,
    pub update: Upd,
    pub dialogue: Dialogue<State, T>,
}

impl<Upd, State, T> GetChatId for DialogueHandlerCtx<Upd, State, T>
where
    Upd: GetChatId,
{
    fn chat_id(&self) -> i64 {
        self.update.chat_id()
    }
}

impl<State, T> DialogueHandlerCtx<Message, State, T> {
    pub async fn reply<S>(&self, text: S) -> ResponseResult<()>
    where
        S: Into<String>,
    {
        self.bot
            .send_message(self.chat_id(), text)
            .send()
            .await
            .map(|_| ())
    }
}
