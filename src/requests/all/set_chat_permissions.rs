use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatPermissions, True},
    Bot,
};

/// Use this method to set default chat permissions for all members. The bot
/// must be an administrator in the group or a supergroup for this to work and
/// must have the can_restrict_members admin rights. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPermissions<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    chat_id: ChatId,
    /// New default chat permissions
    permissions: ChatPermissions,
}

#[async_trait::async_trait]
impl Request for SetChatPermissions<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendChatPermissions",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SetChatPermissions<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        permissions: ChatPermissions,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            permissions,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }
}
