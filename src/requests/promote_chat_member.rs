use crate::network;
use crate::requests::{ChatId, Request, RequestContext, ResponseResult};
use crate::types::True;

use async_trait::async_trait;
use std::borrow::Cow;

///Use this method to promote or demote a user in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights. Pass False for all boolean parameters to
/// demote a user. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMember<'a> {
    #[serde(skip_serializing)]
    ctx: RequestContext<'a>,
    ///Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: Cow<'a, ChatId>,
    ///Unique identifier of the target user
    pub user_id: i32,
    ///Pass True, if the administrator can change chat title, photo and other
    /// settings
    #[serde(skip_serializing_if = "Option::is_none")]
    can_change_info: Option<bool>,
    ///Pass True, if the administrator can create channel posts, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_post_messages: Option<bool>,
    ///Pass True, if the administrator can edit messages of other users and
    /// can pin messages, channels only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_edit_messages: Option<bool>,
    ///Pass True, if the administrator can delete messages of other users
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_delete_messages: Option<bool>,
    ///Pass True, if the administrator can invite new users to the chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_invite_users: Option<bool>,
    ///Pass True, if the administrator can restrict, ban or unban chat members
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_restrict_members: Option<bool>,
    ///Pass True, if the administrator can pin messages, supergroups only
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_pin_messages: Option<bool>,
    ///Pass True, if the administrator can add new administrators with a
    /// subset of his own privileges or demote administrators that he has
    /// promoted, directly or indirectly (promoted by administrators that were
    /// appointed by him)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_promote_members: Option<bool>,
}
#[async_trait]
impl Request for PromoteChatMember<'_> {
    type ReturnValue = True;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl PromoteChatMember<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.ctx.client,
            &self.ctx.token,
            "promoteChatMember",
            &self,
        )
        .await
    }
}
impl<'a> PromoteChatMember<'a> {
    pub(crate) fn new<C>(
        ctx: RequestContext<'a>,
        chat_id: C,
        user_id: i32,
    ) -> Self where C: Into<Cow<'a, ChatId>> {
        Self {
            ctx,
            chat_id: chat_id.into(),
            user_id,
            can_change_info: None,
            can_post_messages: None,
            can_edit_messages: None,
            can_delete_messages: None,
            can_invite_users: None,
            can_restrict_members: None,
            can_pin_messages: None,
            can_promote_members: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<Cow<'a, ChatId>>,
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

    pub fn can_change_info<T>(mut self, can_change_info: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_change_info = Some(can_change_info.into());
        self
    }

    pub fn can_post_messages<T>(mut self, can_post_messages: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_post_messages = Some(can_post_messages.into());
        self
    }

    pub fn can_edit_messages<T>(mut self, can_edit_messages: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_edit_messages = Some(can_edit_messages.into());
        self
    }

    pub fn can_delete_messages<T>(mut self, can_delete_messages: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_delete_messages = Some(can_delete_messages.into());
        self
    }

    pub fn can_invite_users<T>(mut self, can_invite_users: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_invite_users = Some(can_invite_users.into());
        self
    }

    pub fn can_restrict_members<T>(mut self, can_restrict_members: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_restrict_members = Some(can_restrict_members.into());
        self
    }

    pub fn can_pin_messages<T>(mut self, can_pin_messages: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_pin_messages = Some(can_pin_messages.into());
        self
    }

    pub fn can_promote_members<T>(mut self, can_promote_members: T) -> Self
    where
        T: Into<bool>,
    {
        self.can_promote_members = Some(can_promote_members.into());
        self
    }
}
