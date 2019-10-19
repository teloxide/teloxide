use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

#[derive(Debug, Clone, Serialize)]
pub struct SetChatTitle<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId,
    title: String,
}

#[async_trait]
impl Request for SetChatTitle<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SetChatTitle<'_> {
    async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.bot.client(),
            &self.bot.token(),
            "setChatTitle",
            &self,
        )
        .await
    }
}

impl<'a> SetChatTitle<'a> {
    pub(crate) fn new<C, T>(
        bot: &'a Bot,
        chat_id: C,
        title: T
    ) -> Self
        where
            C: Into<ChatId>,
            T: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }

    pub fn chat_id<C>(mut self, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn title<C>(mut self, title: C) -> Self
    where
        C: Into<String>,
    {
        self.title = title.into();
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
        let title = "title";
        let method = SetChatTitle::new(&bot, chat_id, title);

        let expected = r#"{"chat_id":123,"title":"title"}"#;
        let actual = serde_json::to_string::<SetChatTitle>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
