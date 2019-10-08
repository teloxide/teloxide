use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::{Message, ParseMode, ReplyMarkup},
};
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize)]
/// Use this method to send text messages. On success, the sent [`Message`] is
/// returned.
pub struct SendMessage<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,

    ///	Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: Cow<'a, ChatId>,
    /// Text of the message to be sent
    pub text: Cow<'a, str>,

    /// Send [Markdown] or [HTML],
    /// if you want Telegram apps to show [bold, italic, fixed-width text
    /// or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<Cow<'a, ReplyMarkup>>,
}

#[async_trait]
impl Request for SendMessage<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendMessage<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.ctx.client,
            self.ctx.token,
            "sendMessage",
            &self,
        )
        .await
    }
}

impl<'a> SendMessage<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        text: String,
    ) -> Self {
        SendMessage {
            ctx,
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<Cow<'a, ChatId>>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn text<T>(mut self, val: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.text = val.into();
        self
    }

    pub fn parse_mode<T>(mut self, val: T) -> Self
    where
        T: Into<ParseMode>,
    {
        self.parse_mode = Some(val.into());
        self
    }

    pub fn disable_web_page_preview<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_web_page_preview = Some(val.into());
        self
    }

    pub fn disable_notification<T>(mut self, val: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(val.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(val.into());
        self
    }

    pub fn reply_markup<T>(mut self, val: T) -> Self
    where
        T: Into<Cow<'a, ReplyMarkup>>,
    {
        self.reply_markup = Some(val.into());
        self
    }
}
