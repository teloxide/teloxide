use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatPermissions, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to set default chat permissions for all members.
///
/// The bot must be an administrator in the group or a supergroup for this to
/// work and must have the can_restrict_members admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatpermissions).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissions {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    permissions: ChatPermissions,
}

#[async_trait::async_trait]
impl Request for SetChatPermissions {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendChatPermissions",
            &self,
        )
        .await
    }
}

impl SetChatPermissions {
    pub(crate) fn new<C>(
        bot: Arc<Bot>,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, permissions }
    }

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format `@supergroupusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// New default chat permissions.
    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }
}
