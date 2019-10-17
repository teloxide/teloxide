use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InlineKeyboardMarkup, Message},
};

/// Use this method to stop updating a live location message before live_period
/// expires. On success, if the message was sent by the bot, the sent Message is
/// returned, otherwise True is returned.
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Required if inline_message_id is not specified. Unique identifier for
    /// the target chat or username of the target channel (in the format
    /// @channelusername)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_id: Option<ChatId>,
    /// Required if inline_message_id is not specified. Identifier of the
    /// message with live location to stop
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i32>,
    /// Required if chat_id and message_id are not specified. Identifier of the
    /// inline message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,
    /// A JSON-serialized object InlineKeyboardMarkup for a new inline
    /// keyboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<InlineKeyboardMarkup>,
}

#[async_trait]
impl Request for StopMessageLiveLocation<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl StopMessageLiveLocation<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "stopMessageLiveLocation",
            &self,
        )
        .await
    }
}

impl<'a> StopMessageLiveLocation<'a> {
    pub(crate) fn new(bot: &'a Bot) -> Self {
        Self {
            bot,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = Some(value.into());
        self
    }

    pub fn message_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.message_id = Some(value.into());
        self
    }

    pub fn inline_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(value.into());
        self
    }

    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
