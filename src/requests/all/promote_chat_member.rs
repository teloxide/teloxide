use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to promote or demote a user in a supergroup or a channel.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the appropriate admin rights. Pass False for all boolean parameters to
/// demote a user.
///
/// [The official docs](https://core.telegram.org/bots/api#promotechatmember).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct PromoteChatMember {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    user_id: i32,
    can_change_info: Option<bool>,
    can_post_messages: Option<bool>,
    can_edit_messages: Option<bool>,
    can_delete_messages: Option<bool>,
    can_invite_users: Option<bool>,
    can_restrict_members: Option<bool>,
    can_pin_messages: Option<bool>,
    can_promote_members: Option<bool>,
}

#[async_trait::async_trait]
impl Request for PromoteChatMember {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "promoteChatMember",
            &self,
        )
        .await
    }
}

impl PromoteChatMember {
    pub(crate) fn new<C>(bot: Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
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

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
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

    /// Pass `true`, if the administrator can change chat title, photo and other
    /// settings.
    pub fn can_change_info(mut self, val: bool) -> Self {
        self.can_change_info = Some(val);
        self
    }

    /// Pass `true`, if the administrator can create channel posts, channels
    /// only.
    pub fn can_post_messages(mut self, val: bool) -> Self {
        self.can_post_messages = Some(val);
        self
    }

    /// Pass `true`, if the administrator can edit messages of other users and
    /// can pin messages, channels only.
    pub fn can_edit_messages(mut self, val: bool) -> Self {
        self.can_edit_messages = Some(val);
        self
    }

    /// Pass `true`, if the administrator can delete messages of other users.
    pub fn can_delete_messages(mut self, val: bool) -> Self {
        self.can_delete_messages = Some(val);
        self
    }

    /// Pass `true`, if the administrator can invite new users to the chat.
    pub fn can_invite_users(mut self, val: bool) -> Self {
        self.can_invite_users = Some(val);
        self
    }

    /// Pass `true`, if the administrator can restrict, ban or unban chat
    /// members.
    pub fn can_restrict_members(mut self, val: bool) -> Self {
        self.can_restrict_members = Some(val);
        self
    }

    /// Pass `true`, if the administrator can pin messages, supergroups only.
    pub fn can_pin_messages(mut self, val: bool) -> Self {
        self.can_pin_messages = Some(val);
        self
    }

    /// Pass `true`, if the administrator can add new administrators with a
    /// subset of his own privileges or demote administrators that he has
    /// promoted, directly or indirectly (promoted by administrators that were
    /// appointed by him).
    pub fn can_promote_members(mut self, val: bool) -> Self {
        self.can_promote_members = Some(val);
        self
    }
}
