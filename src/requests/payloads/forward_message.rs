use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, Message},
};

/// Use this method to forward messages of any kind. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct ForwardMessage {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier for the chat where the original message was sent (or channel username in the format @channelusername)
    from_chat_id: ChatId,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// Message identifier in the chat specified in from_chat_id
    message_id: i32,
}

impl Method for ForwardMessage {
    type Output = Message;

    const NAME: &'static str = "forwardMessage";
}

impl json::Payload for ForwardMessage {}

impl dynamic::Payload for ForwardMessage {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl ForwardMessage {
    pub fn new<C, F>(chat_id: C, from_chat_id: F, message_id: i32) -> Self
    where
        C: Into<ChatId>,
        F: Into<ChatId>
    {
        let chat_id = chat_id.into();
        let from_chat_id = from_chat_id.into();
        Self {
            chat_id,
            from_chat_id,
            message_id,
            disable_notification: None,
        }
    }
}

impl json::Request<'_, ForwardMessage> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    #[allow(clippy::wrong_self_convention)]
    pub fn from_chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.from_chat_id = val.into();
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.payload.disable_notification = Some(val);
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.payload.message_id = val;
        self
    }
}
                 