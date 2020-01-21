use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to unban a previously kicked user in a supergroup or
/// channel. The user will **not** return to the group or channel automatically,
/// but will be able to join via link, etc. The bot must be an administrator for
/// this to work.
///
/// [The official docs](https://core.telegram.org/bots/api#unbanchatmember).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    chat_id: ChatId,
    user_id: i32,
}

#[async_trait::async_trait]
impl Request for UnbanChatMember<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
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

    /// Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format `@username`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }
}
