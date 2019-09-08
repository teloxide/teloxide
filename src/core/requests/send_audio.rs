use crate::core::requests::{ChatId, Request, RequestFuture, ResponseResult, RequestContext};
use crate::core::types::{InputFile, ParseMode, Message};
use crate::core::requests::form_builder::FormBuilder;
use crate::core::network;

pub struct SendAudio<'a> {
    ctx: RequestContext<'a>,

    chat_id: ChatId,
    audio: InputFile,
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    duration: Option<i32>,
    performer: Option<String>,
    title: Option<String>,
    thumb: Option<InputFile>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
    reply_markup: Option<()> // TODO: add reply_markup
}

impl<'a> Request<'a> for SendAudio<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let mut params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add_if_some("caption", self.caption.as_ref())
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some("duration", self.duration.as_ref())
                .add_if_some("performer", self.performer.as_ref())
                .add_if_some("title", self.title.as_ref())
                .add_if_some(
                    "disable_notification",
                    self.disable_notification.as_ref()
                )
                .add_if_some(
                    "reply_to_message_id",
                    self.reply_to_message_id.as_ref()
                );
            params = match self.audio {
                InputFile::File(file) => params.add_file("audio", &file),
                InputFile::Url(url) => params.add("audio", &url),
                InputFile::FileId(file_id) => params.add("audio", &file_id),
            };
            if self.thumb.is_some() {
                params = match self.thumb.unwrap() {
                    InputFile::File(file) => params.add_file("thumb", &file),
                    InputFile::Url(url) => params.add("thumb", &url),
                    InputFile::FileId(file_id) => params.add("thumb", &file_id),
                };
            }
            let params = params.build();

            network::request_multipart(
                &self.ctx.client,
                &self.ctx.token,
                "sendAudio",
                Some(params)
            ).await
        })
    }
}

impl<'a> SendAudio<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        audio: InputFile,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            audio,
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn audio<T: Into<InputFile>>(mut self, audio: T) -> Self {
        self.audio = audio.into();
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

    pub fn duration<T: Into<i32>>(mut self, duration: T) -> Self {
        self.duration = Some(duration.into());
        self
    }

    pub fn performer<T: Into<String>>(mut self, performer: T) -> Self {
        self.performer = Some(performer.into());
        self
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn thumb<T: Into<InputFile>>(mut self, thumb: T) -> Self {
        self.thumb = Some(thumb.into());
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
