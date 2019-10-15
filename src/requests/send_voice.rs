use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::{ChatId, Message, ParseMode, ReplyMarkup},
};

///Use this method to send audio files, if you want Telegram clients to display
/// the file as a playable voice message. For this to work, your audio must be
/// in an .ogg file encoded with OPUS (other formats may be sent as Audio or
/// Document). On success, the sent Message is returned. Bots can currently send
/// voice messages of up to 50 MB in size, this limit may be changed in the
/// future.
#[derive(Debug, Clone, Serialize)]
pub struct SendVoice<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists
    /// on the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    pub voice: String,
    //InputFile or String
    /// Voice message caption, 0-1024 characters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    ///	Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    ///Duration of the voice message in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<u64>,
    ///    Sends the message silently. Users will receive a notification with
    /// no sound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<i32>,
    /// InlineKeyboardMarkup or ReplyKeyboardMarkup or ReplyKeyboardRemove or
    /// ForceReply 	Optional 	Additional interface options. A JSON-serialized
    /// object for an inline keyboard, custom reply keyboard, instructions to
    /// remove reply keyboard or to force a reply from the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

#[async_trait]
impl Request for SendVoice<'_> {
    type Output = Message;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SendVoice<'_> {
    pub async fn send(self) -> ResponseResult<Message> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "sendVoice",
            &self,
        )
        .await
    }
}

impl<'a> SendVoice<'a> {
    pub(crate) fn new<C, V>(
        ctx: RequestContext<'a>,
        chat_id: C,
        voice: V,
    ) -> Self
    where
        C: Into<ChatId>,
        V: Into<String>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            voice: voice.into(),
            caption: None,
            parse_mode: None,
            duration: None,
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

    pub fn voice<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.voice = value.into();
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

    pub fn duration<T>(mut self, value: T) -> Self
    where
        T: Into<u64>,
    {
        self.duration = Some(value.into());
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
