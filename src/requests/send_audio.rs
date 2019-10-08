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
    pub chat_id: ChatId<'a>,
    /// Audio to send.
    /// [`InputFile::FileId`] - Pass a file_id as String to send an audio that
    /// exists on the Telegram servers (recommended).
    /// [`InputFile::Url`] - Pass an HTTP URL as a String for Telegram
    /// to get an audio from the Internet.
    /// [`InputFile::File`] - Upload a new audio.
    pub audio: InputFile<'a>,
    /// Audio caption, 0-1024 characters
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
    /// Duration of the audio in seconds
    pub duration: Option<i32>,
    /// Performer
    pub performer: Option<Cow<'a, str>>,
    /// Track name
    pub title: Option<Cow<'a, str>>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>
    pub thumb: Option<InputFile<'a>>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup<'a>>,
}

#[async_trait]
impl Request for SendAudio<'_> {
    type ReturnValue = Message<'static>;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendAudio<'_> {
    pub async fn send(self) -> ResponseResult<Message<'static>> {
        let mut params = FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add_if_some("caption", self.caption.as_deref())
            .add_if_some("parse_mode", self.parse_mode.as_ref())
            .add_if_some("duration", self.duration.as_ref())
            .add_if_some("performer", self.performer.as_deref())
            .add_if_some("title", self.title.as_deref())
            .add_if_some(
                "disable_notification",
                self.disable_notification.as_ref(),
            )
            .add_if_some(
                "reply_to_message_id",
                self.reply_to_message_id.as_ref(),
            );
        params = match self.audio {
            InputFile::File(file) => params.add_file("audio", &file),
            InputFile::Url(url) => params.add("audio", url.deref()),
            InputFile::FileId(file_id) => params.add("audio", file_id.deref()),
        };
        if let Some(thumb) = self.thumb {
            params = match thumb {
                InputFile::File(file) => params.add_file("thumb", &file),
                InputFile::Url(url) => params.add("thumb", url.deref()),
                InputFile::FileId(file_id) => {
                    params.add("thumb", file_id.deref())
                }
            }
        }
        let params = params.build();

        network::request_multipart(
            &self.ctx.client,
            &self.ctx.token,
            "sendAudio",
            Some(params),
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
        C: Into<ChatId<'a>>,
        A: Into<InputFile<'a>>,
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

    pub fn chat_id<C>(mut self, chat_id: C) -> Self
    where
        C: Into<ChatId<'a>>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn audio<A>(mut self, audio: A) -> Self
    where
        A: Into<InputFile<'a>>,
    {
        self.audio = audio.into();
        self
    }

    pub fn caption<C>(mut self, caption: C) -> Self
    where
        C: Into<Cow<'a, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode<P>(mut self, parse_mode: P) -> Self
    where
        P: Into<ParseMode>,
    {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    pub fn duration<D>(mut self, duration: D) -> Self
    where
        D: Into<i32>,
    {
        self.duration = Some(duration.into());
        self
    }

    pub fn performer<P>(mut self, performer: P) -> Self
    where
        P: Into<Cow<'a, str>>,
    {
        self.performer = Some(performer.into());
        self
    }

    pub fn title<T>(mut self, title: T) -> Self
    where
        T: Into<Cow<'a, str>>,
    {
        self.title = Some(title.into());
        self
    }

    pub fn thumb<T>(mut self, thumb: T) -> Self
    where
        T: Into<InputFile<'a>>,
    {
        self.thumb = Some(thumb.into());
        self
    }

    pub fn disable_notification<D>(mut self, disable_notification: D) -> Self
    where
        D: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<R>(mut self, reply_to_message_id: R) -> Self
    where
        R: Into<i32>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}
