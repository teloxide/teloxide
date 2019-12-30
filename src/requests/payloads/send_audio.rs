use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, multipart, Method, form_builder::FormBuilder},
    types::{ReplyMarkup, InputFile, ParseMode, ChatId, Message},
};

/// Use this method to send audio files, if you want Telegram clients to display them in the music player. Your audio must be in the .MP3 or .M4A format. On success, the sent Message is returned. Bots can currently send audio files of up to 50 MB in size, this limit may be changed in the future.For sending voice messages, use the sendVoice method instead.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendAudio {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Audio file to send. Pass a file_id as String to send an audio file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get an audio file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    audio: InputFile,
    /// Audio caption, 0-1024 characters
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// Duration of the audio in seconds
    duration: Option<i32>,
    /// Performer
    performer: Option<String>,
    /// Track name
    title: Option<String>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for the file is supported server-side. The thumbnail should be in JPEG format and less than 200 kB in size. A thumbnail‘s width and height should not exceed 320. Ignored if the file is not uploaded using multipart/form-data. Thumbnails can’t be reused and can be only uploaded as a new file, so you can pass “attach://<file_attach_name>” if the thumbnail was uploaded using multipart/form-data under <file_attach_name>. More info on Sending Files »
    thumb: Option<InputFile>,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl Method for SendAudio {
    type Output = Message;

    const NAME: &'static str = "sendAudio";
}

impl multipart::Payload for SendAudio {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add("audio", &self.audio)
            .add("caption", &self.caption)
            .add("parse_mode", &self.parse_mode)
            .add("duration", &self.duration)
            .add("performer", &self.performer)
            .add("title", &self.title)
            .add("thumb", &self.thumb)
            .add("disable_notification", &self.disable_notification)
            .add("reply_to_message_id", &self.reply_to_message_id)
            .add("reply_markup", &self.reply_markup)
            .build()
    }
}

impl dynamic::Payload for SendAudio {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl SendAudio {
    pub fn new<C, A>(chat_id: C, audio: A) -> Self
    where
        C: Into<ChatId>,
        A: Into<InputFile>
    {
        let chat_id = chat_id.into();
        let audio = audio.into();
        Self {
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
            reply_markup: None,
        }
    }
}

impl multipart::Request<'_, SendAudio> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn audio<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.audio = val.into();
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

    pub fn duration(mut self, val: i32) -> Self {
        self.payload.duration = Some(val);
        self
    }

    pub fn performer<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.performer = Some(val.into());
        self
    }

    pub fn title<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.title = Some(val.into());
        self
    }

    pub fn thumb<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.thumb = Some(val.into());
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
