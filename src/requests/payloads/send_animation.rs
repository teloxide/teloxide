use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, form_builder::FormBuilder, multipart, Method},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
};

/// Use this method to send animation files (GIF or H.264/MPEG-4 AVC video
/// without sound).
///
/// On success, the sent Message is returned.
///
/// Bots can currently send animation files of up to 50 MB in size, this limit
/// may be changed in the future.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendAnimation {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`)
    pub chat_id: ChatId,
    /// Animation to send.
    pub animation: InputFile,
    /// Duration of sent animation in seconds
    pub duration: Option<u32>,
    /// Animation width
    pub width: Option<u32>,
    /// Animation height
    pub height: Option<u32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// [`InputFile::File`]. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, with [`InputFile::File`]
    ///
    /// [`InputFile::File`]: crate::types::InputFile::File
    pub thumb: Option<InputFile>,
    /// Animation caption, `0`-`1024` characters
    pub caption: Option<String>,
    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    pub disable_notification: Option<bool>,
    /// If the message is a reply, [id] of the original message
    ///
    /// [id]: crate::types::Message::id
    pub reply_to_message_id: Option<i32>,
    /// Additional interface options
    pub reply_markup: Option<ReplyMarkup>,
}

impl Method for SendAnimation {
    type Output = Message;

    const NAME: &'static str = "sendAnimation";
}

impl multipart::Payload for SendAnimation {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add("animation", &self.animation)
            .add("duration", &self.duration)
            .add("width", &self.width)
            .add("height", &self.height)
            .add("thumb", &self.thumb)
            .add("caption", &self.caption)
            .add("parse_mode", &self.parse_mode)
            .add("disable_notification", &self.disable_notification)
            .add("reply_to_message_id", &self.reply_to_message_id)
            .add("reply_markup", &self.reply_markup)
            .build()
    }
}

impl dynamic::Payload for SendAnimation {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl SendAnimation {
    pub fn new<C>(chat_id: C, animation: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
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
}

impl multipart::Request<'_, SendAnimation> {
    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.payload.chat_id = value.into();
        self
    }

    pub fn duration(mut self, value: u32) -> Self {
        self.payload.duration = Some(value);
        self
    }

    pub fn width(mut self, value: u32) -> Self {
        self.payload.width = Some(value);
        self
    }
    pub fn height(mut self, value: u32) -> Self {
        self.payload.height = Some(value);
        self
    }
    pub fn thumb(mut self, value: InputFile) -> Self {
        self.payload.thumb = Some(value);
        self
    }

    pub fn caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.payload.caption = Some(value.into());
        self
    }
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.payload.parse_mode = Some(value);
        self
    }
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.payload.disable_notification = Some(value);
        self
    }
    pub fn reply_to_message_id(mut self, value: i32) -> Self {
        self.payload.reply_to_message_id = Some(value);
        self
    }
    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.payload.reply_markup = Some(value.into());
        self
    }
}
