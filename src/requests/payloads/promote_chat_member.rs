use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};

/// Use this method to promote or demote a user in a supergroup or a channel. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Pass False for all boolean parameters to demote a user. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct PromoteChatMember {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
    /// Pass True, if the administrator can change chat title, photo and other settings
    can_change_info: Option<bool>,
    /// Pass True, if the administrator can create channel posts, channels only
    can_post_messages: Option<bool>,
    /// Pass True, if the administrator can edit messages of other users and can pin messages, channels only
    can_edit_messages: Option<bool>,
    /// Pass True, if the administrator can delete messages of other users
    can_delete_messages: Option<bool>,
    /// Pass True, if the administrator can invite new users to the chat
    can_invite_users: Option<bool>,
    /// Pass True, if the administrator can restrict, ban or unban chat members
    can_restrict_members: Option<bool>,
    /// Pass True, if the administrator can pin messages, supergroups only
    can_pin_messages: Option<bool>,
    /// Pass True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by him)
    can_promote_members: Option<bool>,
}

impl Method for PromoteChatMember {
    type Output = True;

    const NAME: &'static str = "promoteChatMember";
}

impl json::Payload for PromoteChatMember {}

impl dynamic::Payload for PromoteChatMember {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl PromoteChatMember {
    pub fn new<C>(chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
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
}

impl json::Request<'_, PromoteChatMember> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn can_change_info(mut self, val: bool) -> Self {
        self.payload.can_change_info = Some(val);
        self
    }

    pub fn can_post_messages(mut self, val: bool) -> Self {
        self.payload.can_post_messages = Some(val);
        self
    }

    pub fn can_edit_messages(mut self, val: bool) -> Self {
        self.payload.can_edit_messages = Some(val);
        self
    }

    pub fn can_delete_messages(mut self, val: bool) -> Self {
        self.payload.can_delete_messages = Some(val);
        self
    }

    pub fn can_invite_users(mut self, val: bool) -> Self {
        self.payload.can_invite_users = Some(val);
        self
    }

    pub fn can_restrict_members(mut self, val: bool) -> Self {
        self.payload.can_restrict_members = Some(val);
        self
    }

    pub fn can_pin_messages(mut self, val: bool) -> Self {
        self.payload.can_pin_messages = Some(val);
        self
    }

    pub fn can_promote_members(mut self, val: bool) -> Self {
        self.payload.can_promote_members = Some(val);
        self
    }
}
                 