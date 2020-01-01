use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{True, ChatId},
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

impl Method for SetChatAdministratorCustomTitle {
    type Output = True;

    const NAME: &'static str = "setChatAdministratorCustomTitle";
}

impl json::Payload for SetChatAdministratorCustomTitle {}

impl dynamic::Payload for SetChatAdministratorCustomTitle {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
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
}

impl json::Request<'_, SetChatAdministratorCustomTitle> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn user_id(mut self, val: i32) -> Self {
        self.payload.user_id = val;
        self
    }

    pub fn custom_title<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.payload.custom_title = val.into();
        self
    }
}
