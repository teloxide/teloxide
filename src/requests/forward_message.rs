use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::Message,
};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
/// Use this method to forward messages of any kind. On success, the sent
/// [`Message`] is returned.
pub struct ForwardMessage<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: Cow<'a, ChatId>,
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub from_chat_id: Cow<'a, ChatId>,
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
    pub(crate) fn new<C>(
        ctx: RequestContext<'a>,
        chat_id: C,
        from_chat_id: C,
        message_id: i32,
    ) -> Self
    where
        C: Into<Cow<'a, ChatId>>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            from_chat_id: from_chat_id.into(),
            message_id,
            disable_notification: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<Cow<'a, ChatId>>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn from_chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<Cow<'a, ChatId>>,
    {
        self.from_chat_id = val.into();
        self
    }

    pub fn message_id<T>(mut self, val: T) -> Self
    where
        T: Into<i32>,
    {
        self.message_id = val.into();
        self
    }

    pub fn disable_notification<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(val.into());
        self
    }
}
