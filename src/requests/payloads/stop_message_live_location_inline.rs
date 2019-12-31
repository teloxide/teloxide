use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{InlineKeyboardMarkup, Message},
};

/// Use this method to stop updating a live location message before live_period expires. On success, if the message was sent by the bot, the sent Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct StopMessageLiveLocationInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for StopMessageLiveLocationInline {
    type Output = Message;

    const NAME: &'static str = "stopMessageLiveLocation";
}

impl json::Payload for StopMessageLiveLocationInline {}

impl dynamic::Payload for StopMessageLiveLocationInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl StopMessageLiveLocationInline {
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

impl json::Request<'_, StopMessageLiveLocationInline> {
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
