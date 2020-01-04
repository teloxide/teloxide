use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, Message, ReplyMarkup},
    Bot,
};

/// As of v.4.0, Telegram clients support rounded square mp4 videos of up to 1
/// minute long. Use this method to send video messages. On success, the sent
/// Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendVideoNote<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Video note to send. Pass a file_id as String to send a video note that
    /// exists on the Telegram servers (recommended) or upload a new video
    /// using multipart/form-data. More info on Sending Files ». Sending video
    /// notes by a URL is currently unsupported
    video_note: InputFile,
    /// Duration of sent video in seconds
    duration: Option<i32>,
    /// Video width and height, i.e. diameter of the video message
    length: Option<i32>,
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
    /// the file is supported server-side. The thumbnail should be in JPEG
    /// format and less than 200 kB in size. A thumbnail‘s width and height
    /// should not exceed 320. Ignored if the file is not uploaded using
    /// multipart/form-data. Thumbnails can’t be reused and can be only
    /// uploaded as a new file, so you can pass “attach://<file_attach_name>”
    /// if the thumbnail was uploaded using multipart/form-data under
    /// <file_attach_name>. More info on Sending Files »
    thumb: Option<InputFile>,
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
impl Request for SendVideoNote<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendVideoNote",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("video_note", &self.video_note)
                .await
                .add("duration", &self.duration)
                .await
                .add("length", &self.length)
                .await
                .add("thumb", &self.thumb)
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

impl<'a> SendVideoNote<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        video_note: InputFile,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            video_note,
            duration: None,
            length: None,
            thumb: None,
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

    pub fn video_note(mut self, val: InputFile) -> Self {
        self.video_note = val;
        self
    }

    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
        self
    }

    pub fn length(mut self, val: i32) -> Self {
        self.length = Some(val);
        self
    }

    pub fn thumb(mut self, val: InputFile) -> Self {
        self.thumb = Some(val);
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
