use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ChatId, Message},
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
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
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
    pub(crate) fn new<C, Fc, M>(
        ctx: RequestContext<'a>,
        chat_id: C,
        from_chat_id: Fc,
        message_id: M,
    ) -> Self
    where
        C: Into<ChatId>,
        Fc: Into<ChatId>,
        M: Into<i32>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id: message_id.into(),
            disable_notification: None,
        }
    }

    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.from_chat_id = value.into();
        self
    }

    pub fn message_id<M>(mut self, value: M) -> Self
    where
        M: Into<i32>,
    {
        self.message_id = value.into();
        self
    }

    pub fn disable_notification<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.disable_notification = Some(value.into());
        self
    }
}
