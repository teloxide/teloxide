use crate::network;
use crate::requests::{
    ChatId, Request, RequestContext, RequestFuture, ResponseResult,
};

/// Use this method to kick a user from a group, a supergroup or a channel. In
/// the case of supergroups and channels, the user will not be able to return to
/// the group on their own using invite links, etc., unless unbanned first. The
/// bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i32,
    ///Date when the user will be unbanned, unix time. If user is banned for
    /// more than 366 days or less than 30 seconds from the current time they
    /// are considered to be banned forever
    pub until_date: Option<u64>,
}

impl<'a> Request<'a> for KickChatMember<'a> {
    type ReturnValue = bool;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                self.ctx.client,
                self.ctx.token,
                "kickChatMember",
                &self,
            )
            .await
        })
    }
}

impl<'a> KickChatMember<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        user_id: i32,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            user_id,
            until_date: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn user_id<T: Into<i32>>(mut self, user_id: T) -> Self {
        self.user_id = user_id.into();
        self
    }

    pub fn until_date<T: Into<u64>>(mut self, until_date: T) -> Self {
        self.until_date = Some(user_id.into());
        self
    }
}
