use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{InlineKeyboardMarkup, Message},
};

/// Use this method to edit only the reply markup of messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageReplyMarkupInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageReplyMarkupInline {
    type Output = Message;

    const NAME: &'static str = "editMessageReplyMarkup";
}

impl json::Payload for EditMessageReplyMarkupInline {}

impl dynamic::Payload for EditMessageReplyMarkupInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageReplyMarkupInline {
    pub fn new<I>(inline_message_id: I) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageReplyMarkupInline> {
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_message_id = val.into();
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
