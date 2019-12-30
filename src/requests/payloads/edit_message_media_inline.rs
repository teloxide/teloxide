use serde::{Deserialize, Serialize};
use reqwest::multipart::Form;

use crate::{
    requests::{dynamic, multipart, Method, form_builder::FormBuilder},
    types::{InputMedia, InlineKeyboardMarkup, Message},
};

/// Use this method to edit animation, audio, document, photo, or video messages. If a message is a part of a message album, then it can be edited only to a photo or a video. Otherwise, message type can be changed arbitrarily. When inline message is edited, new file can't be uploaded. Use previously uploaded file via its file_id or specify a URL. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct EditMessageMediaInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// A JSON-serialized object for a new media content of the message
    media: InputMedia,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageMediaInline {
    type Output = Message;

    const NAME: &'static str = "editMessageMedia";
}

impl multipart::Payload for EditMessageMediaInline {
    fn payload(&self) -> Form {
        FormBuilder::new()
            .add("inline_message_id", &self.inline_message_id)
            .add("media", &self.media)
            .add("reply_markup", &self.reply_markup)
            .build()
    }
}

impl dynamic::Payload for EditMessageMediaInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Multipart(multipart::Payload::payload(self))
    }
}

impl EditMessageMediaInline {
    pub fn new<I>(inline_message_id: I, media: InputMedia) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            media,
            reply_markup: None,
        }
    }
}

impl multipart::Request<'_, EditMessageMediaInline> {
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_message_id = val.into();
        self
    }

    pub fn media(mut self, val: InputMedia) -> Self {
        self.payload.media = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
