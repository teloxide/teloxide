use async_trait::async_trait;

use crate::network;
use crate::requests::{Request, RequestContext, ResponseResult};
use crate::types::{ChatId, Message, ParseMode, ReplyMarkup};

//TODO: add action to bot api
///Use this method to send video files, Telegram clients support mp4 videos
/// (other formats may be sent as Document). On success, the sent Message is
/// returned. Bots can currently send video files of up to 50 MB in size, this
/// limit may be changed in the future.
#[derive(Debug, Clone, Serialize)]
pub struct SendVideo<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    ///Video to send. Pass a file_id as String to send a video that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a video from the Internet, or upload a new video using
    /// multipart/form-data. More info on Sending Files »
    pub video: String,
    ///Duration of sent video in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    ///Video width
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i32>,
    ///Video height
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i32>,
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
    //InputFile or String
    ///Video caption (may also be used when resending videos by file_id),
    /// 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    ///Pass True, if the uploaded video is suitable for streaming
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_streaming: Option<bool>,
    ///	Sends the message silently. Users will receive a notification with no
    /// sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    ///If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    ///Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove reply keyboard
    /// or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendVideo<'_> {
    type ReturnValue = Message;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendVideo<'_> {
    async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendVideo",
            &self,
        )
        .await
    }
}

impl<'a> SendVideo<'a> {
    pub(crate) fn new<C, V>(
        ctx: RequestContext<'a>,
        chat_id: C,
        video: V,
    ) -> Self
    where
        C: Into<ChatId>,
        V: Into<String>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            video: video.into(),
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
    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn video<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.video = value.into();
        self
    }

    pub fn duration<T>(mut self, value: T) -> Self
    where
        T: Into<u64>,
    {
        self.duration = Some(value.into());
        self
    }
    pub fn width<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.width = Some(value.into());
        self
    }
    pub fn height<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.height = Some(value.into());
        self
    }
    pub fn thumb<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumb = Some(value.into());
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
    pub fn supports_streaming<T>(mut self, value: T) -> Self
    where
        T: Into<bool>,
    {
        self.supports_streaming = Some(value.into());
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
