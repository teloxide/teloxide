use async_trait::async_trait;

use crate::bot::Bot;
use crate::types::{ChatId, True};
use crate::requests::{ResponseResult, Request};
use crate::network;

#[derive(Debug, Clone, Serialize)]
pub struct DeleteChatPhoto<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId
}

#[async_trait]
impl Request for DeleteChatPhoto<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl DeleteChatPhoto<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "deleteChatPhoto",
            &self
        ).await
    }
}

impl<'a> DeleteChatPhoto<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>
    {
        Self {
            bot,
            chat_id: chat_id.into(),
        }
    }

    pub fn chat_id<C>(mut self, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        let bot = Bot::new("token");
        let chat_id = 123;
        let method = DeleteChatPhoto::new(&bot, chat_id);

        let expected = r#"{"chat_id":123}"#;
        let actual = serde_json::to_string::<DeleteChatPhoto>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
