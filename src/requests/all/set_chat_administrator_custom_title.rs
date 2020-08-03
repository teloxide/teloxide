use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};

/// Use this method to set a custom title for an administrator in a supergroup
/// promoted by the bot.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatadministratorcustomtitle).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatAdministratorCustomTitle {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    user_id: i32,
    custom_title: String,
}

#[async_trait::async_trait]
impl Request for SetChatAdministratorCustomTitle {
    type Output = True;

    async fn send(self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatAdministratorCustomTitle",
            &self,
        )
        .await
    }
}

impl SetChatAdministratorCustomTitle {
    pub(crate) fn new<C, CT>(bot: Bot, chat_id: C, user_id: i32, custom_title: CT) -> Self
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        let chat_id = chat_id.into();
        let custom_title = custom_title.into();
        Self { bot, chat_id, user_id, custom_title }
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

    /// New custom title for the administrator; 0-16 characters, emoji are not
    /// allowed.
    pub fn custom_title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.custom_title = val.into();
        self
    }
}
