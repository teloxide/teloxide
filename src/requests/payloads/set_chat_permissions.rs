use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, ChatPermissions, True},
};

/// Use this method to set default chat permissions for all members. The bot must be an administrator in the group or a supergroup for this to work and must have the can_restrict_members admin rights. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetChatPermission {
    /// Unique identifier for the target chat or username of the target supergroup (in the format @supergroupusername)
    chat_id: ChatId,
    /// New default chat permissions
    permissions: ChatPermissions,
}

impl Method for SetChatPermission {
    type Output = True;

    const NAME: &'static str = "setChatPermissions";
}

impl json::Payload for SetChatPermission {}

impl dynamic::Payload for SetChatPermission {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SetChatPermission {
    pub fn new<C>(chat_id: C, permissions: ChatPermissions) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            permissions,
        }
    }
}

impl json::Request<'_, SetChatPermission> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.payload.permissions = val;
        self
    }
}
                 