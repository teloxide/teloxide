use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, InputFile, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to set a new profile photo for the chat.
///
/// Photos can't be changed for private chats. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchatphoto).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPhoto {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    photo: InputFile,
}

#[async_trait::async_trait]
impl Request for SetChatPhoto {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatPhoto",
            &self,
        )
        .await
    }
}

impl SetChatPhoto {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C, photo: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, photo }
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

    /// New chat photo, uploaded using `multipart/form-data`.
    pub fn photo(mut self, val: InputFile) -> Self {
        self.photo = val;
        self
    }
}
