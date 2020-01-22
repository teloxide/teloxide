use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send video files, Telegram clients support mp4 videos
/// (other formats may be sent as Document). On success, the sent Message is
/// returned. Bots can currently send video files of up to 50 MB in size, this
/// limit may be changed in the future.
#[derive(Debug, Clone)]
pub struct SendVideo<'a> {
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Video to send. Pass a file_id as String to send a video that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a video from the Internet, or upload a new video using
    /// multipart/form-data. More info on Sending Files »
    video: InputFile,
    /// Duration of sent video in seconds
    duration: Option<i32>,
    /// Video width
    width: Option<i32>,
    /// Video height
    height: Option<i32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>. More info on Sending Files »
    thumb: Option<InputFile>,
    /// Video caption (may also be used when resending videos by file_id),
    /// 0-1024 characters
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// Pass True, if the uploaded video is suitable for streaming
    supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendVideo<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendVideo",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("video", &self.video)
                .await
                .add("duration", &self.duration)
                .await
                .add("width", &self.width)
                .await
                .add("height", &self.height)
                .await
                .add("thumb", &self.thumb)
                .await
                .add("caption", &self.caption)
                .await
                .add("parse_mode", &self.parse_mode)
                .await
                .add("supports_streaming", &self.supports_streaming)
                .await
                .add("disable_notification", &self.disable_notification)
                .await
                .add("reply_to_message_id", &self.reply_to_message_id)
                .await
                .add("reply_markup", &self.reply_markup)
                .await
                .build(),
        )
        .await
    }
}

impl<'a> SendVideo<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, video: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            video,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            supports_streaming: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn video(mut self, val: InputFile) -> Self {
        self.video = val;
        self
    }

    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn width(mut self, val: i32) -> Self {
        self.width = Some(val);
        self
    }

    pub fn height(mut self, val: i32) -> Self {
        self.height = Some(val);
        self
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
        self
    }

    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.parse_mode = Some(val);
        self
    }

    pub fn supports_streaming(mut self, val: bool) -> Self {
        self.supports_streaming = Some(val);
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
