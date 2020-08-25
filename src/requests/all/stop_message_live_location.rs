use serde::Serialize;

use crate::{
    net,
    requests::{RequestOld, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, Message},
    Bot,
};

/// Use this method to stop updating a live location message before
/// `live_period` expires.
///
/// On success, the sent [`Message`] is returned.
///
/// [The official docs](https://core.telegram.org/bots/api#stopmessagelivelocation).
///
/// [`Message`]: crate::types::Message
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocation {
    #[serde(skip_serializing)]
    bot: Bot,
    pub chat_id: ChatId,
    pub message_id: i32,
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait::async_trait]
impl RequestOld for StopMessageLiveLocation {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "stopMessageLiveLocation", &self)
            .await
    }
}

impl StopMessageLiveLocation {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, message_id, reply_markup: None }
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

    /// A JSON-serialized object for a new [inline keyboard].
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
