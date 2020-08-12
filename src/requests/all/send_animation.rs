use crate::{
    net,
    requests::{form_builder::FormBuilder, RequestWithFile, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video
/// without sound).
///
/// Bots can currently send animation files of up to 50 MB in size, this limit
/// may be changed in the future.
///
/// [The official docs](https://core.telegram.org/bots/api#sendanimation).
#[derive(Debug, Clone)]
pub struct SendAnimation {
    bot: Bot,
    pub chat_id: ChatId,
    pub animation: InputFile,
    pub duration: Option<u32>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl RequestWithFile for SendAnimation {
    type Output = Message;

    async fn send(&self) -> tokio::io::Result<ResponseResult<Message>> {
        let mut builder = FormBuilder::new()
            .add_text("chat_id", &self.chat_id)
            .add_input_file("animation", &self.animation)
            .await?
            .add_text("duration", &self.duration)
            .add_text("width", &self.width)
            .add_text("height", &self.height)
            .add_text("caption", &self.caption)
            .add_text("parse_mode", &self.parse_mode)
            .add_text("disable_notification", &self.disable_notification)
            .add_text("reply_to_message_id", &self.reply_to_message_id)
            .add_text("reply_markup", &self.reply_markup);
        if let Some(thumb) = self.thumb.as_ref() {
            builder = builder.add_input_file("thumb", thumb).await?;
        }
        Ok(net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendAnimation",
            builder.build(),
        )
        .await)
    }
}

impl SendAnimation {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, animation: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            animation,
            duration: None,
            width: None,
            height: None,
            thumb: None,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    /// Animation to send.
    pub fn animation(mut self, val: InputFile) -> Self {
        self.animation = val;
        self
    }

    /// Duration of sent animation in seconds.
    pub fn duration(mut self, value: u32) -> Self {
        self.duration = Some(value);
        self
    }

    /// Animation width.
    pub fn width(mut self, value: u32) -> Self {
        self.width = Some(value);
        self
    }

    /// Animation height.
    pub fn height(mut self, value: u32) -> Self {
        self.height = Some(value);
        self
    }

    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side.
    ///
    /// The thumbnail should be in JPEG format and less than 200 kB in size. A
    /// thumbnail‘s width and height should not exceed 320. Ignored if the
    /// file is not uploaded using [`InputFile::File`]. Thumbnails can’t be
    /// reused and can be only uploaded as a new file, with
    /// [`InputFile::File`].
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    pub fn thumb(mut self, value: InputFile) -> Self {
        self.thumb = Some(value);
        self
    }

    /// Animation caption, `0`-`1024` characters.
    pub fn caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }

    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// If the message is a reply, [id] of the original message.
    ///
    /// [id]: crate::types::Message::id
    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }

    /// Additional interface options.
    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
