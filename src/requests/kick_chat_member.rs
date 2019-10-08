use async_trait::async_trait;

use crate::{
    network,
    requests::{ChatId, Request, RequestContext, ResponseResult},
    types::True,
};
use std::borrow::Cow;

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
    pub chat_id: Cow<'a, ChatId>,
    /// Unique identifier of the target user
    pub user_id: i32,
    ///Date when the user will be unbanned, unix time. If user is banned for
    /// more than 366 days or less than 30 seconds from the current time they
    /// are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[async_trait]
impl<'a> Request for KickChatMember<'a> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl KickChatMember<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.ctx.client,
            self.ctx.token,
            "kickChatMember",
            &self,
        )
        .await
    }
}

impl<'a> KickChatMember<'a> {
    pub(crate) fn new<C>(
        ctx: RequestContext<'a>,
        chat_id: C,
        user_id: i32,
    ) -> Self where C: Into<Cow<'a, ChatId>> {
        Self {
            ctx,
            chat_id: chat_id.into(),
            user_id,
            until_date: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self where T: Into<Cow<'a, ChatId>> {
        self.chat_id = chat_id.into();
        self
    }

    pub fn user_id<T>(mut self, user_id: T) -> Self where T: Into<i32> {
        self.user_id = user_id.into();
        self
    }

    pub fn until_date<T>(mut self, until_date: T) -> Self where T: Into<u64> {
        self.until_date = Some(until_date.into());
        self
    }
}
