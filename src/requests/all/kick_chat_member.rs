use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to kick a user from a group, a supergroup or a channel. In
/// the case of supergroups and channels, the user will not be able to return to
/// the group on their own using invite links, etc., unless unbanned first. The
/// bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
    /// Unique identifier of the target user
    user_id: i32,
    /// Date when the user will be unbanned, unix time. If user is banned for
    /// more than 366 days or less than 30 seconds from the current time they
    /// are considered to be banned forever
    until_date: Option<i32>,
}

#[async_trait::async_trait]
impl Request for KickChatMember<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "kickChatMember",
            &self,
        )
        .await
    }
}

impl<'a> KickChatMember<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, user_id: i32) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            user_id,
            until_date: None,
        }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.user_id = val;
        self
    }

    pub fn until_date(mut self, val: i32) -> Self {
        self.until_date = Some(val);
        self
    }
}
