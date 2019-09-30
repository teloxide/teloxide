use crate::network;
use crate::requests::{
    ChatId, Request, RequestContext, RequestFuture, ResponseResult,
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

impl<'a> Request<'a> for UnbanChatMember<'a> {
    type ReturnValue = bool;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "unbanChatMember",
                &self,
            )
            .await
        })
    }
}


impl<'a>UnbanChatMember<'a>{

    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        user_id: i32,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            user_id,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
        where
        T: Into<ChatId>,
        {
        self.chat_id = chat_id.into();
        self
    }


    pub fn user_id<T>(mut self, user_id: T) -> Self
        where
            T: Into<i32>,
    {
        self.user_id = user_id.into();
        self
    }

}