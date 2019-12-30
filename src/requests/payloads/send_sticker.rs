use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, multipart, Method, form_builder::FormBuilder},
    types::{ReplyMarkup, InputFile, ChatId, Message},
};

/// Use this method to send static .WEBP or animated .TGS stickers. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendSticker {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Sticker to send. Pass a file_id as String to send a file that exists on the Telegram servers (recommended), pass an HTTP URL as a String for Telegram to get a .webp file from the Internet, or upload a new one using multipart/form-data. More info on Sending Files »
    sticker: InputFile,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove reply keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl Method for SendSticker {
    type Output = Message;

    const NAME: &'static str = "sendSticker";
}

impl multipart::Payload for SendSticker {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .add("sticker", &self.sticker)
            .add("disable_notification", &self.disable_notification)
            .add("reply_to_message_id", &self.reply_to_message_id)
            .add("reply_markup", &self.reply_markup)
            .build()
    }
}

impl dynamic::Payload for SendSticker {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl SendSticker {
    pub fn new<C, S>(chat_id: C, sticker: S) -> Self
    where
        C: Into<ChatId>,
        S: Into<InputFile>
    {
        let chat_id = chat_id.into();
        let sticker = sticker.into();
        Self {
            chat_id,
            sticker,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
}

impl multipart::Request<'_, SendSticker> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn sticker<T>(mut self, val: T) -> Self
    where
        T: Into<InputFile>
    {
        self.payload.sticker = val.into();
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
