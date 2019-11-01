use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
};

#[derive(Debug, Clone, Serialize)]
pub struct SetChatDescription<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

#[async_trait]
impl Request for SetChatDescription<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SetChatDescription<'_> {
    pub async fn send(self) -> ResponseResult<True> {
        network::request_json(
            &self.bot.client(),
            &self.bot.token(),
            "setChatDescription",
            &self,
        )
        .await
    }
}

impl<'a> SetChatDescription<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            description: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_new() {
        let bot = Bot::new("token");
        let chat_id = 123;
        let method = SetChatDescription::new(&bot, chat_id);

        let expected = r#"{"chat_id":123}"#;
        let actual =
            serde_json::to_string::<SetChatDescription>(&method).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn serialize_description() {
        let bot = Bot::new("token");
        let chat_id = 123;
        let description = "description";
        let method =
            SetChatDescription::new(&bot, chat_id).description(description);

        let expected = r#"{"chat_id":123,"description":"description"}"#;
        let actual =
            serde_json::to_string::<SetChatDescription>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
