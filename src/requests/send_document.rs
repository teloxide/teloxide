use async_trait::async_trait;

use crate::{
    network,
    requests::{Request,  ResponseResult},
    types::{ChatId, Message, ParseMode, ReplyMarkup},
};
use crate::bot::Bot;

// TODO: add method to bot/api

///Use this method to send general files. On success, the sent Message is
/// returned. Bots can currently send files of any type of up to 50 MB in size,
/// this limit may be changed in the future.
#[derive(Debug, Clone, Serialize)]
pub struct SendDocument<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    /// Unique identifier for the target chat or username of the target
    /// channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// File to send. Pass a file_id as String to send a file that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data.»
    pub document: String,
    //InputFile or String
    /// Thumbnail of the file sent; can be ignored if thumbnail generation for
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
    /// Document caption (may also be used when resending documents by
    /// file_id), 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    /// Sends the message silently. Users will receive a notification with
    /// no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendDocument<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendDocument<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendDocument",
            &self,
        )
        .await
    }
}

impl<'a> SendDocument<'a> {
    pub(crate) fn new<C, D>(
        bot: &'a Bot,
        chat_id: C,
        document: D,
    ) -> Self
    where
        C: Into<ChatId>,
        D: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            document: document.into(),
            thumb: None,
            caption: None,
            parse_mode: None,
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

    pub fn document<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.document = value.into();
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
