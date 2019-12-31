use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{InlineKeyboardMarkup, Message},
};

/// Use this method to edit live location messages. A location can be edited until its live_period expires or editing is explicitly disabled by a call to stopMessageLiveLocation. On success, if the edited message was sent by the bot, the edited Message is returned, otherwise True is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub struct EditMessageLiveLocationInline {
    /// Identifier of the inline message
    inline_message_id: String,
    /// Latitude of new location
    latitude: f32,
    /// Longitude of new location
    longitude: f32,
    /// A JSON-serialized object for a new inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for EditMessageLiveLocationInline {
    type Output = Message;

    const NAME: &'static str = "editMessageLiveLocation";
}

impl json::Payload for EditMessageLiveLocationInline {}

impl dynamic::Payload for EditMessageLiveLocationInline {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl EditMessageLiveLocationInline {
    pub fn new<I>(inline_message_id: I, latitude: f32, longitude: f32) -> Self
    where
        I: Into<String>,
    {
        let inline_message_id = inline_message_id.into();
        Self {
            inline_message_id,
            latitude,
            longitude,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, EditMessageLiveLocationInline> {
    pub fn inline_message_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.inline_message_id = val.into();
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
