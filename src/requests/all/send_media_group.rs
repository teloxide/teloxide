use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputMedia, Message},
    Bot,
};

/// Use this method to send a group of photos or videos as an album. On success,
/// an array of the sent Messages is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendMediaGroup<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// A JSON-serialized array describing photos and videos to be sent, must
    /// include 2–10 items
    media: Vec<InputMedia>, // TODO: InputMediaPhoto and InputMediaVideo
    /// Sends the messages silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the messages are a reply, ID of the original message
    reply_to_message_id: Option<i32>,
}

#[async_trait::async_trait]
impl Request for SendMediaGroup<'_> {
    type Output = Vec<Message>;

    async fn send(&self) -> ResponseResult<Vec<Message>> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendMediaGroup",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("media", &self.media)
                .await
                .add("disable_notification", &self.disable_notification)
                .await
                .add("reply_to_message_id", &self.reply_to_message_id)
                .await
                .build(),
        )
        .await
    }
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new<C, M>(bot: &'a Bot, chat_id: C, media: M) -> Self
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        let chat_id = chat_id.into();
        let media = media.into();
        Self {
            bot,
            chat_id,
            media,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn media<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<InputMedia>>,
    {
        self.media = val.into();
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }
}
