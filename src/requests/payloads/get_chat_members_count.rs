use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::ChatId,
};

/// Use this method to get the number of members in a chat. Returns Int on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct GetChatMembersCount {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

impl Method for GetChatMembersCount {
    type Output = i32;

    const NAME: &'static str = "getChatMembersCount";
}

impl json::Payload for GetChatMembersCount {}

impl dynamic::Payload for GetChatMembersCount {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl GetChatMembersCount {
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

impl json::Request<'_, GetChatMembersCount> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }
}
                 