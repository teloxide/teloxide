use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, multipart, Method, form_builder::FormBuilder},
    types::{ReplyMarkup, InputFile, Message, ParseMode, ChatId},
};

/// Use this method to send video files, Telegram clients support mp4 videos (other formats may be sent as Document). On success, the sent Message is returned. Bots can currently send video files of up to 50 MB in size, this limit may be changed in the future.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendVideo {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Video to send. Pass a file_id as String to send a video that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a video from the Internet, or upload a new video using multipart/form-data. More info on Sending Files »
    video: InputFile,
    /// Duration of sent video in seconds
    duration: Option<i32>,
    /// Video width
    width: Option<i32>,
    /// Video height
    height: Option<i32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    thumb: Option<InputFile>,
    /// Video caption (may also be used when resending videos by file_id), 0-1024 characters
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// Pass True, if the uploaded video is suitable for streaming
    supports_streaming: Option<bool>,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl Method for SendVideo {
    type Output = Message;

    const NAME: &'static str = "sendVideo";
}

impl multipart::Payload for SendVideo {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add("video", &self.video)
            .add("duration", &self.duration)
            .add("width", &self.width)
            .add("height", &self.height)
            .add("thumb", &self.thumb)
            .add("caption", &self.caption)
            .add("parse_mode", &self.parse_mode)
            .add("supports_streaming", &self.supports_streaming)
            .add("disable_notification", &self.disable_notification)
            .add("reply_to_message_id", &self.reply_to_message_id)
            .add("reply_markup", &self.reply_markup)
            .build()
    }
}

impl dynamic::Payload for SendVideo {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl SendVideo {
    pub fn new<C, V>(chat_id: C, video: V) -> Self
    where
        C: Into<ChatId>,
        V: Into<InputFile>
    {
        let chat_id = chat_id.into();
        let video = video.into();
        Self {
            chat_id,
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
}

impl multipart::Request<'_, SendVideo> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn video<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.video = val.into();
        self
    }

    pub fn duration(mut self, val: i32) -> Self {
        self.payload.duration = Some(val);
        self
    }

    pub fn width(mut self, val: i32) -> Self {
        self.payload.width = Some(val);
        self
    }

    pub fn height(mut self, val: i32) -> Self {
        self.payload.height = Some(val);
        self
    }

    pub fn thumb<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.thumb = Some(val.into());
        self
    }

    pub fn caption<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.caption = Some(val.into());
        self
    }

    pub fn parse_mode(mut self, val: ParseMode) -> Self {
        self.payload.parse_mode = Some(val);
        self
    }

    pub fn supports_streaming(mut self, val: bool) -> Self {
        self.payload.supports_streaming = Some(val);
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.payload.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.payload.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
