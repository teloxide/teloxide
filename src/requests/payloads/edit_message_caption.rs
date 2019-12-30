use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, ParseMode, InlineKeyboardMarkup, Message},
};

/// Use this method to edit captions of messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageCaption {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// New caption of the message
    caption: Option<String>,
    /// Send Markdown or HTML, if you want Telegram apps to show bold, italic, fixed-width text or inline URLs in the media caption.
    parse_mode: Option<ParseMode>,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageCaption {
    type Output = Message;

    const NAME: &'static str = "editMessageCaptionInline";
}

impl json::Payload for EditMessageCaption {}

impl dynamic::Payload for EditMessageCaption {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageCaption {
    pub fn new<C>(chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            caption: None,
            parse_mode: None,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageCaption> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.payload.message_id = val;
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

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
