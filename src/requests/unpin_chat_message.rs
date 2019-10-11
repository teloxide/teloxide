use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ChatId, True}
};

#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessage<'a> {
    #[serde(skip_serializing)]
    pub ctx: RequestContext<'a>,

    pub chat_id: ChatId,
}

#[async_trait]
impl Request for UnpinChatMessage<'_> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl UnpinChatMessage<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "unpinChatMessage",
            &self,
        )
        .await
    }
}

impl<'a> UnpinChatMessage<'a> {
    pub(crate) fn new(ctx: RequestContext<'a>, chat_id: ChatId) -> Self {
        Self { ctx, chat_id }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }
}
