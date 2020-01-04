use serde::Serialize;

use crate::{
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send static .WEBP or animated .TGS stickers. On success,
/// the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendSticker<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Sticker to send. Pass a file_id as String to send a file that exists on
    /// the Telegram servers (recommended), pass an HTTP URL as a String for
    /// Telegram to get a .webp file from the Internet, or upload a new one
    /// using multipart/form-data. More info on Sending Files »
    sticker: InputFile,
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
impl Request for SendSticker<'_> {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendSticker",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("sticker", &self.sticker)
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

impl<'a> SendSticker<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, sticker: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            sticker,
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

    pub fn sticker(mut self, val: InputFile) -> Self {
        self.sticker = val;
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
