use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};

/// Use this method to kick a user from a group, a supergroup or a channel. In the case of supergroups and channels, the user will not be able to return to the group on their own using invite links, etc., unless unbanned first. The bot must be an administrator in the chat for this to work and must have the appropriate admin rights. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct KickChatMember {
    /// Unique identifier for the target group or username of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
    /// Date when the user will be unbanned, unix time. If user is banned for more than 366 days or less than 30 seconds from the current time they are considered to be banned forever
    until_date: Option<i32>,
}

impl Method for KickChatMember {
    type Output = True;

    const NAME: &'static str = "kickChatMember";
}

impl json::Payload for KickChatMember {}

impl dynamic::Payload for KickChatMember {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl KickChatMember {
    pub fn new<C>(chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            user_id,
            until_date: None,
        }
    }
}

impl json::Request<'_, KickChatMember> {
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

    pub fn until_date(mut self, val: i32) -> Self {
        self.payload.until_date = Some(val);
        self
    }
}
                 