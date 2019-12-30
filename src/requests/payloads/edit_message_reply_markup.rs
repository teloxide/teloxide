use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, InlineKeyboardMarkup, Message},
};

/// Use this method to edit only the reply markup of messages. On success, if edited message is sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct EditMessageReplyMarkup {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// A JSON-serialized object for an inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageReplyMarkup {
    type Output = Message;

    const NAME: &'static str = "editMessageReplyMarkupInline";
}

impl json::Payload for EditMessageReplyMarkup {}

impl dynamic::Payload for EditMessageReplyMarkup {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageReplyMarkup {
    pub fn new<C>(chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageReplyMarkup> {
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

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
