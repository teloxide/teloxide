use serde::Serialize;

use super::BotWrapper;
use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send a native poll. A native poll can't be sent to a
/// private chat.
///
/// [The official docs](https://core.telegram.org/bots/api#sendpoll).
#[serde_with_macros::skip_serializing_none]
#[derive(Eq, PartialEq, Debug, Clone, Serialize)]
pub struct SendPoll<'a> {
    #[serde(skip_serializing)]
    bot: BotWrapper<'a>,
    chat_id: ChatId,
    question: String,
    options: Vec<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendPoll<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendPoll",
            &self,
        )
        .await
    }
}

impl<'a> SendPoll<'a> {
    pub(crate) fn new<C, Q, O>(
        bot: &'a Bot,
        chat_id: C,
        question: Q,
        options: O,
    ) -> Self
    where
        C: Into<ChatId>,
        Q: Into<String>,
        O: Into<Vec<String>>,
    {
        let chat_id = chat_id.into();
        let question = question.into();
        let options = options.into();
        Self {
            bot: BotWrapper(bot),
            chat_id,
            question,
            options,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    ///
    /// A native poll can't be sent to a private chat.
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Poll question, 1-255 characters.
    pub fn question<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.question = val.into();
        self
    }

    /// List of answer options, 2-10 strings 1-100 characters each.
    pub fn options<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.options = val.into();
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

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
