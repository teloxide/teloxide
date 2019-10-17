use async_trait::async_trait;

use crate::{
    network,
    requests::{Request,  ResponseResult},
    types::{ChatId, True},
};
use crate::bot::Bot;

/// Use this method to kick a user from a group, a supergroup or a channel. In
/// the case of supergroups and channels, the user will not be able to return to
/// the group on their own using invite links, etc., unless unbanned first. The
/// bot must be an administrator in the chat for this to work and must have the
/// appropriate admin rights. Returns True on success.
#[derive(Debug, Clone, Serialize)]
pub struct KickChatMember<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,
    ///Unique identifier for the target group or username of the target
    /// supergroup or channel (in the format @channelusername)
    pub chat_id: ChatId,
    /// Unique identifier of the target user
    pub user_id: i32,
    ///Date when the user will be unbanned, unix time. If user is banned for
    /// more than 366 days or less than 30 seconds from the current time they
    /// are considered to be banned forever
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_date: Option<u64>,
}

#[async_trait]
impl Request for KickChatMember<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl KickChatMember<'_> {
    async fn send(self) -> ResponseResult<True> {
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
    pub(crate) fn new<C, U>(
        bot: &'a Bot,
        chat_id: C,
        user_id: U,
    ) -> Self
    where
        C: Into<ChatId>,
        U: Into<i32>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            user_id: user_id.into(),
            until_date: None,
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

    pub fn until_date<T>(mut self, value: T) -> Self
    where
        T: Into<u64>,
    {
        self.until_date = Some(value.into());
        self
    }
}
