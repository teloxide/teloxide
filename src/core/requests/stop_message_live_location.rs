use crate::core::{
    network,
    requests::{
        ChatId,
        Request,
        RequestFuture,
        RequestContext,
        ResponseResult,
    },
    types::{InlineKeyboardMarkup, Message},
};

/// Use this method to stop updating a live location message before live_period
/// expires. On success, if the message was sent by the bot, the sent Message is
/// returned, otherwise True is returned.
#[derive(Debug, Clone, Serialize)]
pub struct StopMessageLiveLocation<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
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

impl<'a> Request<'a> for StopMessageLiveLocation<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "stopMessageLiveLocation",
                &self,
            )
            .await
        })
    }
}

impl<'a> StopMessageLiveLocation<'a> {
    fn new(ctx: RequestContext<'a>) -> Self {
        Self {
            ctx,
            chat_id: None,
            message_id: None,
            inline_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = Some(chat_id.into());
        self
    }

    pub fn message_id<T>(mut self, message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.message_id = Some(message_id.into());
        self
    }

    pub fn inline_message_id<T>(mut self, inline_message_id: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_message_id = Some(inline_message_id.into());
        self
    }

    pub fn reply_markup<T>(mut self, reply_markup: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}
