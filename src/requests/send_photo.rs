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
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        photo: InputFile,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn photo<T: Into<InputFile>>(mut self, photo: T) -> Self {
        self.photo = photo.into();
        self
    }

    pub fn caption<T: Into<String>>(mut self, caption: T) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode<T: Into<ParseMode>>(mut self, parse_mode: T) -> Self {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    pub fn disable_notification<T: Into<bool>>(
        mut self,
        disable_notification: T,
    ) -> Self {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i32>>(
        mut self,
        reply_to_message_id: T,
    ) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}
