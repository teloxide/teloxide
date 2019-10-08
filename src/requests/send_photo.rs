use std::borrow::Cow;
use std::ops::Deref;

use async_trait::async_trait;

use crate::{
    network,
    requests::{
        form_builder::FormBuilder, ChatId, Request, RequestContext,
        ResponseResult,
    },
    types::{InputFile, Message, ParseMode, ReplyMarkup},
};

#[derive(Debug, Clone)]
/// Use this method to send photos. On success, the sent [`Message`] is
/// returned.
pub struct SendPhoto<'a> {
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId<'a>,
    /// Photo to send.
    /// [`InputFile::FileId`] - Pass a file_id as String to send a photo that
    /// exists on the Telegram servers (recommended)
    /// [`InputFile::Url`] - Pass an HTTP URL as a String for Telegram
    /// to get a photo from the Internet
    /// [`InputFile::File`] - Upload a new photo.
    pub photo: InputFile<'a>,
    /// Photo caption (may also be used when resending photos by file_id),
    /// 0-1024 characters
    pub caption: Option<Cow<'a, str>>,
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
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

#[async_trait]
impl Request for SendPhoto<'_> {
    type ReturnValue = Message<'static>;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendPhoto<'_> {
    pub async fn send(self) -> ResponseResult<Message<'static>> {
        let mut params = FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add_if_some("caption", self.caption.as_deref())
            .add_if_some("parse_mode", self.parse_mode.as_ref())
            .add_if_some(
                "disable_notification",
                self.disable_notification.as_ref(),
            )
            .add_if_some(
                "reply_to_message_id",
                self.reply_to_message_id.as_ref(),
            );

        params = match self.photo {
            InputFile::File(path) => params.add_file("photo", &path),
            InputFile::Url(url) => params.add("photo", url.deref()),
            InputFile::FileId(file_id) => params.add("photo", file_id.deref()),
        };
        let params = params.build();

        network::request_multipart(
            &self.ctx.client,
            &self.ctx.token,
            "sendPhoto",
            Some(params),
        )
        .await
    }
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new<C, F>(
        ctx: RequestContext<'a>,
        chat_id: C,
        photo: InputFile,
    ) -> Self
    where
        C: Into<ChatId<'a>>,
        F: Into<InputFile<'a>>,
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

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId<'a>>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn photo<T>(mut self, photo: T) -> Self
    where
        T: Into<InputFile<'a>>,
    {
        self.photo = photo.into();
        self
    }

    pub fn caption<T>(mut self, caption: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode<T>(mut self, parse_mode: T) -> Self
    where
        T: Into<Cow<'a, ParseMode>>,
    {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, reply_to_message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}
