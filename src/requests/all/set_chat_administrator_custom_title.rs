use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

/// Use this method to set a custom title for an administrator in a supergroup
/// promoted by the bot.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetChatAdministratorCustomTitle {
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
impl Request<True> for SetChatAdministratorCustomTitle {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "setChatAdministratorCustomTitle",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl SetChatAdministratorCustomTitle {
    pub fn new<C, CT>(chat_id: C, user_id: i32, custom_title: CT) -> Self
    where
        C: Into<ChatId>,
        CT: Into<String>,
    {
        let chat_id = chat_id.into();
        let custom_title = custom_title.into();
        Self {
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
