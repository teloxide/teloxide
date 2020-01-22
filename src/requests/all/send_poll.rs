use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send a native poll. A native poll can't be sent to a
/// private chat. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendPoll<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername). A native poll can't be sent to a
    /// private chat.
    chat_id: ChatId,
    /// Poll question, 1-255 characters
    question: String,
    /// List of answer options, 2-10 strings 1-100 characters each
    options: Vec<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
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
            bot,
            chat_id,
            question,
            options,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn question<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.question = val.into();
        self
    }

    pub fn options<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.options = val.into();
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

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
