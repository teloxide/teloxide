use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{Request, ResponseResult},
    types::ChatId,
};

#[derive(Debug, Clone, Serialize)]
pub struct ExportCharInviteLink<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId,
}

#[async_trait]
impl Request for ExportCharInviteLink<'_> {
    type Output = String;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl ExportCharInviteLink<'_> {
    async fn send(self) -> ResponseResult<String> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "exportChatInviteLink",
            &self,
        )
        .await
    }
}

impl<'a> ExportCharInviteLink<'a> {
    pub(crate) fn new<C>(bot: &'a Bot, chat_id: C) -> Self
    where
        C: Into<ChatId>,
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
        let method = ExportCharInviteLink::new(&bot, chat_id);

        let expected = r#"{"chat_id":123}"#;
        let actual =
            serde_json::to_string::<ExportCharInviteLink>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
