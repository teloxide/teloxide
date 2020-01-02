use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to set a custom title for an administrator in a supergroup
/// promoted by the bot.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatAdministratorCustomTitle<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format `@supergroupusername`)
    pub chat_id: ChatId,

    /// Unique identifier of the target user
    pub user_id: i32,

    /// New custom title for the administrator; 0-16 characters, emoji are not
    /// allowed
    pub custom_title: String,
}

#[async_trait::async_trait]
impl Request<True> for SetChatAdministratorCustomTitle<'_> {
    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatAdministratorCustomTitle",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SetChatAdministratorCustomTitle<'a> {
    pub(crate) fn new<C, CT>(
        bot: &'a Bot,
        chat_id: C,
        user_id: i32,
        custom_title: CT,
    ) -> Self
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        let chat_id = chat_id.into();
        let custom_title = custom_title.into();
        Self {
            bot,
            chat_id,
            user_id,
            custom_title,
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

    pub fn custom_title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_title = val.into();
        self
    }
}
