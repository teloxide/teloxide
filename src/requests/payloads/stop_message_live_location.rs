use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, InlineKeyboardMarkup, Message},
};

/// Use this method to stop updating a live location message before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct StopMessageLiveLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message with live location to stop
    message_id: i32,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for StopMessageLiveLocation {
    type Output = Message;

    const NAME: &'static str = "stopMessageLiveLocationInline";
}

impl json::Payload for StopMessageLiveLocation {}

impl dynamic::Payload for StopMessageLiveLocation {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl StopMessageLiveLocation {
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

impl json::Request<'_, StopMessageLiveLocation> {
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
