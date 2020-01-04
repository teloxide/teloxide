use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, Message, ParseMode, ReplyMarkup},
    Bot,
};

/// Use this method to send audio files, if you want Telegram clients to display
/// the file as a playable voice message. For this to work, your audio must be
/// in an .ogg file encoded with OPUS (other formats may be sent as Audio or
/// Document). On success, the sent Message is returned. Bots can currently send
/// voice messages of up to 50 MB in size, this limit may be changed in the
/// future.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendVoice<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Audio file to send. Pass a file_id as String to send a file that exists
    /// on the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a file from the Internet, or upload a new one using
    /// multipart/form-data. More info on Sending Files »
    voice: InputFile,
    /// Voice message caption, 0-1024 characters
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic,
    /// fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// Duration of the voice message in seconds
    duration: Option<i32>,
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
impl Request for SendVoice<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendVoice",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("voice", &self.voice)
                .await
                .add("caption", &self.caption)
                .await
                .add("parse_mode", &self.parse_mode)
                .await
                .add("duration", &self.duration)
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

impl<'a> SendVoice<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, voice: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            voice,
            caption: None,
            parse_mode: None,
            duration: None,
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

    pub fn voice(mut self, val: InputFile) -> Self {
        self.voice = val;
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

    pub fn duration(mut self, val: i32) -> Self {
        self.duration = Some(val);
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
