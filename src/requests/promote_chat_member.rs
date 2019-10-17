use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

///Use this method to promote or demote a user in a supergroup or a channel.
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights. Pass False for all boolean parameters to
/// demote a user. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    ///Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    pub chat_id: ChatId,
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
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl PromoteChatMember<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "promoteChatMember",
            &self,
        )
        .await
    }
}

impl<'a> PromoteChatMember<'a> {
    pub(crate) fn new<C, U>(bot: &'a Bot, chat_id: C, user_id: U) -> Self
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id: user_id.into(),
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

    pub fn chat_id<C>(mut self, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    pub fn user_id<U>(mut self, value: U) -> Self
    where
        U: Into<i32>,
    {
        self.user_id = value.into();
        self
    }

    pub fn can_change_info<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_change_info = Some(value.into());
        self
    }

    pub fn can_post_messages<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_post_messages = Some(value.into());
        self
    }

    pub fn can_edit_messages<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_edit_messages = Some(value.into());
        self
    }

    pub fn can_delete_messages<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_delete_messages = Some(value.into());
        self
    }

    pub fn can_invite_users<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_invite_users = Some(value.into());
        self
    }

    pub fn can_restrict_members<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_restrict_members = Some(value.into());
        self
    }

    pub fn can_pin_messages<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_pin_messages = Some(value.into());
        self
    }

    pub fn can_promote_members<B>(mut self, value: B) -> Self
    where
        B: Into<bool>,
    {
        self.can_promote_members = Some(value.into());
        self
    }
}
