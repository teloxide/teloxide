use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, ChatMember},
};

/// Use this method to get a list of administrators in a chat. On success, returns an Array of ChatMember objects that contains information about all chat administrators except other bots. If the chat is a group or a supergroup and no administrators were appointed, only the creator will be returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetChatAdministrator {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

impl Method for GetChatAdministrator {
    type Output = Vec<ChatMember>;

    const NAME: &'static str = "getChatAdministrators";
}

impl json::Payload for GetChatAdministrator {}

impl dynamic::Payload for GetChatAdministrator {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetChatAdministrator {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
        }
    }
}

impl json::Request<'_, GetChatAdministrator> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }
}
                 