use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::Message,
};

#[derive(Debug, Clone, Serialize)]
/// Use this method to forward messages of any kind. On success, the sent
/// [`Message`] is returned.
pub struct ForwardMessage<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub from_chat_id: ChatId,
    /// Message identifier in the chat specified in from_chat_id
    pub message_id: i32,

    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
}

#[async_trait]
impl Request for ForwardMessage<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl ForwardMessage<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.ctx.client,
            self.ctx.token,
            "forwardMessage",
            &self,
        )
        .await
    }
}

impl<'a> ForwardMessage<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        from_chat_id: ChatId,
        message_id: i32,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    pub fn from_chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.from_chat_id = val.into();
        self
    }

    pub fn message_id<T: Into<i32>>(mut self, val: T) -> Self {
        self.message_id = val.into();
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }
}
