use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};

/// Use this method to change the description of a group, a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetChatDescription {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// New chat description, 0-255 characters
    description: Option<String>,
}

impl Method for SetChatDescription {
    type Output = True;

    const NAME: &'static str = "setChatDescription";
}

impl json::Payload for SetChatDescription {}

impl dynamic::Payload for SetChatDescription {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetChatDescription {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            description: None,
        }
    }
}

impl json::Request<'_, SetChatDescription> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn description<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.description = Some(val.into());
        self
    }
}
                 