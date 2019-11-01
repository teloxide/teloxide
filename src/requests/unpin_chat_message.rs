use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessage<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    pub chat_id: ChatId,
}

#[async_trait]
impl Request for UnpinChatMessage<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl UnpinChatMessage<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "unpinChatMessage",
            &self,
        )
        .await
    }
}

impl<'a> UnpinChatMessage<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, value: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: value.into(),
        }
    }

    pub fn chat_id<T>(mut self, value: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }
}
