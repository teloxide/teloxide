use crate::network;
use crate::requests::{
    ChatId, Request, RequestContext, RequestFuture, ResponseResult,
};
use crate::types::{ChatPermissions, True};

/// Use this method to restrict a user in a supergroup. The bot must be an
/// administrator in the supergroup for this to work and must have the
/// appropriate admin rights. Pass True for all permissions to lift restrictions
/// from a user. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target chat or username of the target
    /// supergroup (in the format @supergroupusername)
    pub chat_id: ChatId,
    ///Unique identifier of the target user
    pub user_id: i32,
    ///New user permissions
    pub permissions: ChatPermissions,
    ///Date when restrictions will be lifted for the user, unix time. If user
    /// is restricted for more than 366 days or less than 30 seconds from the
    /// current time, they are considered to be restricted forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

impl<'a> Request<'a> for RestrictChatMember<'a> {
    type ReturnValue = True;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            network::request_json(
                &self.ctx.client,
                &self.ctx.token,
                "restrictChatMember",
                &self,
            )
            .await
        })
    }
}

impl<'a> RestrictChatMember<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        user_id: i32,
        permissions: ChatPermissions,
    ) -> Self {
        Self {
            ctx,
            chat_id,
            user_id,
            permissions,
            until_date: None,
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

    pub fn permissions<T>(mut self, permissions: T) -> Self
    where
        T: Into<ChatPermissions>,
    {
        self.permissions = permissions.into();
        self
    }

    pub fn until_date<T>(mut self, until_date: T) -> Self
    where
        T: Into<u64>,
    {
        self.until_date = Some(until_date.into());
        self
    }
}
