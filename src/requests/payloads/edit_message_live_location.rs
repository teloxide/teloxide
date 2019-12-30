use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, InlineKeyboardMarkup, Message},
};

/// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct EditMessageLiveLocation {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the message to edit
    message_id: i32,
    /// Latitude of new location
    latitude: f32,
    /// Longitude of new location
    longitude: f32,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageLiveLocation {
    type Output = Message;

    const NAME: &'static str = "editMessageLiveLocationInline";
}

impl json::Payload for EditMessageLiveLocation {}

impl dynamic::Payload for EditMessageLiveLocation {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageLiveLocation {
    pub fn new<C>(chat_id: C, message_id: i32, latitude: f32, longitude: f32) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageLiveLocation> {
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

    pub fn latitude(mut self, val: f32) -> Self {
        self.payload.latitude = val;
        self
    }

    pub fn longitude(mut self, val: f32) -> Self {
        self.payload.longitude = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
