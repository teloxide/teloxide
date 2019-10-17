use async_trait::async_trait;

use crate::bot::Bot;
use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
};

///As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1
/// minute long. Use this method to send video messages. On success, the sent
/// Message is returned.
#[derive(Debug, Clone, Serialize)]
pub struct SendVideoNote<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    ///Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    ///Video note to send. Pass a file_id as String to send a video note that
    /// exists on the Telegram servers (recommended) or upload a new video
    /// using multipart/form-data. More info on Sending Files ». Sending video
    /// notes by a URL is currently unsupported
    pub video_note: String,
    //	InputFile or String
    ///Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    ///    Video width and height, i.e. diameter of the video message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<u64>,
    ///Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>. More info on Sending Files »
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thumb: Option<String>,
    //	InputFile or String
    ///Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    ///If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    ///    Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendVideoNote<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendVideoNote<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendVideoNote",
            &self,
        )
        .await
    }
}

impl<'a> SendVideoNote<'a> {
    pub(crate) fn new<C, V>(bot: &'a Bot, chat_id: C, video_note: V) -> Self
    where
        C: Into<ChatId>,
        V: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            video_note: video_note.into(),
            duration: None,
            length: None,
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

    pub fn video_note<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.video_note = value.into();
        self
    }

    pub fn duration<T>(mut self, value: T) -> Self
    where
        T: Into<u64>,
    {
        self.duration = Some(value.into());
        self
    }

    pub fn length<T>(mut self, value: T) -> Self
    where
        T: Into<u64>,
    {
        self.length = Some(value.into());
        self
    }

    pub fn thumb<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
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

    pub fn reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}
