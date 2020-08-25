use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, InputMedia, Message},
    Bot,
};

/// Use this method to send a group of photos or videos as an album.
///
/// [The official docs](https://core.telegram.org/bots/api#sendmediagroup).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroup {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub media: Vec<InputMedia>, // TODO: InputMediaPhoto and InputMediaVideo
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
}

#[async_trait::async_trait]
impl RequestOld for SendMediaGroup {
    type Output = Vec<Message>;

    async fn send(&self) -> ResponseResult<Vec<Message>> {
        net::request_multipart(self.bot.client(), self.bot.token(), "sendMediaGroup", self).await
    }
}

impl SendMediaGroup {
    pub(crate) fn new<C, M>(bot: Bot, chat_id: C, media: M) -> Self
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        let chat_id = chat_id.into();
        let media = media.into();
        Self { bot, chat_id, media, disable_notification: None, reply_to_message_id: None }
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

    /// A JSON-serialized array describing photos and videos to be sent, must
    /// include 2–10 items.
    pub fn media<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<InputMedia>>,
    {
        self.media = val.into();
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

    /// If the messages are a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }
}
