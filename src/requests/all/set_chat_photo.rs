use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, InputFile, True},
    Bot,
};

/// Use this method to set a new profile photo for the chat. Photos can't be
/// changed for private chats. The bot must be an administrator in the chat for
/// this to work and must have the appropriate admin rights. Returns True on
/// success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SetChatPhoto<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// New chat photo, uploaded using multipart/form-data
    photo: InputFile,
}

#[async_trait::async_trait]
impl Request for SetChatPhoto<'_> {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "setChatPhoto",
            &self,
        )
        .await
    }
}

impl<'a> SetChatPhoto<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C, photo: InputFile) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self {
            bot,
            chat_id,
            photo,
        }
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
