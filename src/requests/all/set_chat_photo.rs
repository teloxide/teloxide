use serde::{Deserialize, Serialize};

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InputFile, True},
};

/// Use this method to set a new profile photo for the chat. Photos can't be
/// changed for private chats. The bot must be an administrator in the chat for
/// this to work and must have the appropriate admin rights. Returns True on
/// success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SetChatPhoto {
    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data
    photo: InputFile,
}

#[async_trait::async_trait]
impl Request<True> for SetChatPhoto {
    async fn send(&self, bot: &crate::Bot) -> ResponseResult<True> {
        network::request_json(
            bot.client(),
            bot.token(),
            "setChatPhoto",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl SetChatPhoto {
    pub fn new<C>(chat_id: C, photo: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { chat_id, photo }
    }

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn photo(mut self, val: InputFile) -> Self {
        self.photo = val;
        self
    }
}
