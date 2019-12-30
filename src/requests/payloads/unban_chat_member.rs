use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};

/// Use this method to unban a previously kicked user in a supergroup or channel. The user will not return to the group or channel automatically, but will be able to join via link, etc. The bot must be an administrator for this to work. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct UnbanChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @username)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
}

impl Method for UnbanChatMember {
    type Output = True;

    const NAME: &'static str = "unbanChatMember";
}

impl json::Payload for UnbanChatMember {}

impl dynamic::Payload for UnbanChatMember {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl UnbanChatMember {
    pub fn new<C>(chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            user_id,
        }
    }
}

impl json::Request<'_, UnbanChatMember> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }
}
                 