use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatPermissions, True},
    Bot,
};

/// Use this method to restrict a user in a supergroup. The bot must be an
/// administrator in the supergroup for this to work and must have the
/// appropriate admin rights. Pass True for all permissions to lift restrictions
/// from a user. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
    /// New user permissions
    permissions: ChatPermissions,
    /// Date when restrictions will be lifted for the user, unix time. If user
    /// is restricted for more than 366 days or less than 30 seconds from the
    /// current time, they are considered to be restricted forever
    until_date: Option<i32>,
}

#[async_trait::async_trait]
impl Request for RestrictChatMember<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "restrictChatMember",
            &self,
        )
        .await
    }
}

impl<'a> RestrictChatMember<'a> {
    pub(crate) fn new<C>(
        bot: &'a Bot,
        chat_id: C,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            user_id,
            permissions,
            until_date: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }

    pub fn until_date(mut self, val: i32) -> Self {
        self.until_date = Some(val);
        self
    }
}
