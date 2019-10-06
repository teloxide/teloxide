use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::True,
};

/// Use this method to get up to date information about the chat
/// (current name of the user for one-on-one conversations,
/// current username of a user, group or channel, etc.).
/// Returns a Chat object on success.
#[derive(Debug, Clone, Serialize)]
pub struct PinChatMessage<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or username
    /// of the target supergroup or channel (in the format @channelusername)
    pub chat_id: ChatId,
    pub message_id: i32,
    pub disable_notification: Option<bool>,
}

impl<'a> PinChatMessage<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        message_id: i32,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn disable_notification<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(val.into());
        self
    }
}

#[async_trait]
impl<'a> Request for PinChatMessage<'a> {
    type ReturnValue = True;
    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl PinChatMessage<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "pinChatMessage",
            &self,
        )
        .await
    }
}
