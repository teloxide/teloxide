use crate::core::requests::form_builder::FormBuilder;
use crate::core::requests::{
    ChatId, Request, RequestFuture, RequestContext, ResponseResult,
};
use crate::core::{network, types::Message, types::ParseMode};

#[derive(Debug, Clone)]
/// Use this method to send text messages. On success, the sent [`Message`] is returned.
pub struct SendMessage<'a> {
    info: RequestContext<'a>,

    ///	Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Text of the message to be sent
    pub text: String,

    /// Send [Markdown] or [HTML],
    /// if you want Telegram apps to show [bold, italic, fixed-width text
    /// or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::core::types::ParseMode::Markdown
    /// [Html]: crate::core::types::ParseMode::Html
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::core::types::ParseMode
    pub parse_mode: Option<ParseMode>,
    /// Disables link previews for links in this message
    pub disable_web_page_preview: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<()>, // TODO: ReplyMarkup enum
}

impl<'a> Request<'a> for SendMessage<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add::<str>("text", &self.text)
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some(
                    "disable_web_page_preview",
                    self.disable_web_page_preview.as_ref(),
                )
                .add_if_some(
                    "disable_notification",
                    self.disable_notification.as_ref(),
                )
                .add_if_some(
                    "reply_to_message_id",
                    self.reply_to_message_id.as_ref(),
                )
                .build();

            network::request(
                &self.info.client,
                &self.info.token,
                "sendMessage",
                Some(params),
            )
            .await
        })
    }
}

impl<'a> SendMessage<'a> {
    pub(crate) fn new(
        info: RequestContext<'a>,
        chat_id: ChatId,
        text: String,
    ) -> Self {
        SendMessage {
            info,
            chat_id,
            text,
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    pub fn text<T: Into<String>>(mut self, val: T) -> Self {
        self.text = val.into();
        self
    }

    pub fn parse_mode<T: Into<ParseMode>>(mut self, val: T) -> Self {
        self.parse_mode = Some(val.into());
        self
    }

    pub fn disable_web_page_preview<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_web_page_preview = Some(val.into());
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.reply_to_message_id = Some(val.into());
        self
    }

    pub fn reply_markup<T: Into<()>>(mut self, val: T) -> Self {
        self.reply_markup = Some(val.into());
        self
    }
}
