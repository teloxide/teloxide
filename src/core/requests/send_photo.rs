use std::path::Path;

use crate::core::{
    network,
    types::{ParseMode, Message, InputFile},
    requests::{
        ChatId,
        Request,
        RequestFuture,
        RequestContext,
        ResponseResult,
        form_builder::FormBuilder,
    },
};


#[derive(Debug, Clone)]
/// Use this method to send photos. On success, the sent [`Message`] is returned.
pub struct SendPhoto<'a> {
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Photo to send.
    /// [`InputFile::FileId`] - Pass a file_id as String to send a photo that exists on the
    /// Telegram servers (recommended)
    /// [`InputFile::Url`] - Pass an HTTP URL as a String for Telegram
    /// to get a photo from the Internet
    /// [`InputFile::File`] - Upload a new photo.
    pub photo: InputFile,
    /// Photo caption (may also be used when resending photos by file_id), 0-1024 characters
    pub caption: Option<String>,
    /// Send [Markdown] or [HTML],
    /// if you want Telegram apps to show [bold, italic, fixed-width text
    /// or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::core::types::ParseMode::Markdown
    /// [Html]: crate::core::types::ParseMode::Html
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::core::types::ParseMode
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i64>,

    // TODO: add reply_markup
}

impl<'a> Request<'a> for SendPhoto<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let mut params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add_if_some("caption", self.caption.as_ref())
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some(
                    "disable_notification",
                    self.disable_notification.as_ref()
                )
                .add_if_some(
                    "reply_to_message_id",
                    self.reply_to_message_id.as_ref()
                );
            
            params = match self.photo {
                InputFile::File(path) => params.add_file("photo", &path),
                InputFile::Url(url) => params.add("photo", &url),
                InputFile::FileId(file_id) => params.add("photo", &file_id),
            };
            let params = params.build();

            network::request_multipart(
                &self.ctx.client,
                &self.ctx.token,
                "sendPhoto",
                Some(params)
            ).await
        })
    }
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        photo: InputFile
    ) -> Self {
        Self {
            ctx,
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None
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

    pub fn disable_notification<T: Into<bool>>(mut self, disable_notification: T) -> Self {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i64>>(mut self, reply_to_message_id: T) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}
