use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, InputMedia, Message},
    Bot,
};

/// Use this method to edit animation, audio, document, photo, or video
/// messages.
///
/// If a message is a part of a message album, then it can be edited only to a
/// photo or a video. Otherwise, message type can be changed arbitrarily. On
/// success, the edited [`Message`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#editmessagemedia).
///
/// [`Message`]: crate::types::Message
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct EditMessageMedia {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    message_id: i32,
    media: InputMedia,
    reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl RequestOld for EditMessageMedia {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(self.bot.client(), self.bot.token(), "editMessageMedia", self).await
    }
}

impl EditMessageMedia {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, message_id: i32, media: InputMedia) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, message_id, media, reply_markup: None }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`)
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Identifier of the message to edit
    pub fn message_id(mut self, val: i32) -> Self {
        self.message_id = val;
        self
    }

    /// A JSON-serialized object for a new media content of the message.
    pub fn media(mut self, val: InputMedia) -> Self {
        self.media = val;
        self
    }

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
