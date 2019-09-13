use crate::core::network;
use crate::core::requests::{
    ChatId, Request, RequestContext, RequestFuture, ResponseResult,
};
use crate::core::types::{Message, ReplyMarkup};

/// Use this method to send a native poll. A native poll can't be sent to a
/// private chat. On success, the sent Message is returned.
#[derive(Debug, Clone, Serialize)]
struct SendPoll<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// identifier for the target chat or username of the target channel (in
    /// the format @channelusername). A native poll can't be sent to a private
    /// chat.
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
    /// InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove
    /// or ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl<'a> Request<'a> for SendPoll<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "sendPoll",
                &self,
            )
            .await
        })
    }
}

impl<'a> SendPoll<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        question: String,
        options: Vec<String>,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            question,
            options,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn question<T>(mut self, question: T) -> Self
    where
        T: Into<String>,
    {
        self.question = question.into();
        self
    }

    pub fn options<T>(mut self, options: T) -> Self
    where
        T: Into<Vec<String>>,
    {
        self.options = options.into();
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<Vec<bool>>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, reply_to_message_id: T) -> Self
    where
        T: Into<Vec<i32>>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }

    pub fn reply_markup<T>(mut self, reply_markup: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
