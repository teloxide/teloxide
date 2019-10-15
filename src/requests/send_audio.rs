use async_trait::async_trait;

use crate::{
    network,
    requests::{
        form_builder::FormBuilder, Request, RequestContext, ResponseResult,
    },
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
};

/// Use this method to send audio files, if you want Telegram clients to display
/// them in the music player. Your audio must be in the .mp3 format. On success,
/// the sent [`Message`] is returned. Bots can currently send audio files of up
/// to 50 MB in size, this limit may be changed in the future.
///
/// For sending voice messages, use the [`SendVoice`] method instead.
///
/// [`Message`]: crate::types::Message
/// [`SendVoice`]: crate::requests::SendVoice
pub struct SendAudio<'a> {
    ctx: RequestContext<'a>,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Audio to send.
    /// [`InputFile::FileId`] - Pass a file_id as String to send an audio that
    /// exists on the Telegram servers (recommended).
    /// [`InputFile::Url`] - Pass an HTTP URL as a String for Telegram
    /// to get an audio from the Internet.
    /// [`InputFile::File`] - Upload a new audio.
    pub audio: InputFile,
    /// Audio caption, 0-1024 characters
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
    /// Duration of the audio in seconds
    pub duration: Option<i32>,
    /// Performer
    pub performer: Option<String>,
    /// Track name
    pub title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>
    pub thumb: Option<InputFile>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendAudio<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendAudio<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        let params = FormBuilder::new()
            .add("chat_id", self.chat_id)
            .add("caption", self.caption)
            .add("parse_mode", self.parse_mode)
            .add("duration", self.duration)
            .add("performer", self.performer)
            .add("title", self.title)
            .add("disable_notification", self.disable_notification)
            .add("reply_to_message_id", self.reply_to_message_id)
            .add("audio", self.audio)
            .add("thumb", self.thumb);


        network::request_multipart(
            &self.ctx.client,
            &self.ctx.token,
            "sendAudio",
            params.build(),
        )
        .await
    }
}

impl<'a> SendAudio<'a> {
    pub(crate) fn new<C, A>(
        ctx: RequestContext<'a>,
        chat_id: C,
        audio: A,
    ) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            audio: audio.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
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

    pub fn audio<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.audio = value.into();
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

    pub fn duration<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.duration = Some(value.into());
        self
    }

    pub fn performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.performer = Some(value.into());
        self
    }

    pub fn title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }

    pub fn thumb<T>(mut self, value: T) -> Self
    where
        T: Into<InputFile>,
    {
        self.thumb = Some(value.into());
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
