use async_trait::async_trait;

use crate::{
    network,
    requests::{
        form_builder::FormBuilder, Request, RequestContext, ResponseResult,
    },
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
};

#[derive(Debug, Clone)]
/// Use this method to send photos. On success, the sent [`Message`] is
/// returned.
pub struct SendPhoto<'a> {
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Photo to send.
    /// [`InputFile::FileId`] - Pass a file_id as String to send a photo that
    /// exists on the Telegram servers (recommended)
    /// [`InputFile::Url`] - Pass an HTTP URL as a String for Telegram
    /// to get a photo from the Internet
    /// [`InputFile::File`] - Upload a new photo.
    pub photo: InputFile,
    /// Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters
    pub caption: Option<String>,
    /// Send [Markdown] or [HTML],
    /// if you want Telegram apps to show [bold, italic, fixed-width text
    /// or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendPhoto<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendPhoto<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        let params = FormBuilder::new()
            .add("chat_id", self.chat_id)
            .add("caption", self.caption)
            .add("parse_mode", self.parse_mode)
            .add("disable_notification", self.disable_notification)
            .add("reply_to_message_id", self.reply_to_message_id)
            .add("photo", self.photo);

        network::request_multipart(
            &self.ctx.client,
            &self.ctx.token,
            "sendPhoto",
            params.build(),
        )
        .await
    }
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new<C, P>(
        ctx: RequestContext<'a>,
        chat_id: C,
        photo: P,
    ) -> Self
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            photo: photo.into(),
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn photo<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.photo = value.into();
        self
    }

    pub fn caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    pub fn parse_mode<T>(mut self, value: T) -> Self
    where
        T: Into<ParseMode>,
    {
        self.parse_mode = Some(value.into());
        self
    }

    pub fn disable_notification<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(value.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(value.into());
        self
    }
}
