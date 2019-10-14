use async_trait::async_trait;

use crate::{
    network,
    requests::{Request, RequestContext, ResponseResult},
    types::ChatId,
};

/// Use this method to unban a previously kicked user in a supergroup or
/// channel. The user will not return to the group or channel automatically, but
/// will be able to join via link, etc. The bot must be an administrator for
/// this to work. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct UnbanChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i32,
}

#[async_trait]
impl Request for UnbanChatMember<'_> {
    type ReturnValue = bool;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl UnbanChatMember<'_> {
    pub async fn send(self) -> ResponseResult<bool> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "unbanChatMember",
            &self,
        )
        .await
    }
}

impl<'a> UnbanChatMember<'a> {
    pub(crate) fn new<C, U>(
        ctx: RequestContext<'a>,
        chat_id: C,
        user_id: U,
    ) -> Self
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        Self {
            ctx,
            chat_id: chat_id.into(),
            user_id: user_id.into(),
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn user_id<T>(mut self, value: T) -> Self
    where
        T: Into<i32>,
    {
        self.user_id = value.into();
        self
    }
}
