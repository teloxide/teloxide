use async_trait::async_trait;

use crate::{
    network,
    requests::{
        ChatId, Request, RequestContext, RequestFuture, ResponseResult,
    },
    types::Chat,
};

/// Use this method to get up to date information about the chat
/// (current name of the user for one-on-one conversations,
/// current username of a user, group or channel, etc.).
/// Returns a Chat object on success.
#[derive(Debug, Clone, Serialize)]
pub struct GetChat<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or username
    /// of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

#[async_trait]
impl<'a> Request<'a> for GetChat<'a> {
    type ReturnValue = Chat;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue>
    where
        Self: 'a
    {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "getChat",
            &self,
        ).await
    }
}

impl<'a> GetChat<'a> {
    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }
}
