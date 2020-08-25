use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, Message},
    Bot,
};

/// Use this method to forward messages of any kind.
///
/// [`The official docs`](https://core.telegram.org/bots/api#forwardmessage).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct ForwardMessage {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub from_chat_id: ChatId,
    pub disable_notification: Option<bool>,
    pub message_id: i32,
}

#[async_trait::async_trait]
impl RequestOld for ForwardMessage {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "forwardMessage", &self).await
    }
}

impl ForwardMessage {
    pub(crate) fn new<C, F>(bot: Bot, chat_id: C, from_chat_id: F, message_id: i32) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        let from_chat_id = from_chat_id.into();
        Self { bot, chat_id, from_chat_id, message_id, disable_notification: None }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier for the chat where the original message was sent (or
    /// channel username in the format `@channelusername`).
    #[allow(clippy::wrong_self_convention)]
    pub fn from_chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.from_chat_id = val.into();
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// Message identifier in the chat specified in [`from_chat_id`].
    ///
    /// [`from_chat_id`]: ForwardMessage::from_chat_id
    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }
}
