use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to unban a previously kicked user in a supergroup or
/// channel. The user will not return to the group or channel automatically, but
/// will be able to join via link, etc. The bot must be an administrator for
/// this to work. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format @username)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
}

#[async_trait::async_trait]
impl Request for UnbanChatMember<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "unbanChatMember",
            &self,
        )
        .await
    }
}

impl<'a> UnbanChatMember<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            user_id,
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
}
