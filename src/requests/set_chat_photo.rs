use async_trait::async_trait;

use crate::{
    bot::Bot,
    network,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputFile, True},
};

#[derive(Debug, Clone, Serialize)]
pub struct SetChatPhoto<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    chat_id: ChatId,
    photo: InputFile,
}

#[async_trait]
impl Request for SetChatPhoto<'_> {
    type Output = True;

    async fn send_boxed(self) -> ResponseResult<Self::Output> {
        self.send().await
    }
}

impl SetChatPhoto<'_> {
    async fn send(self) -> ResponseResult<True> {
        let params = FormBuilder::new()
            .add("chat_id", self.chat_id)
            .add("photo", self.photo);

        network::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "setChatPhoto",
            params.build(),
        )
        .await
    }
}

impl<'a> SetChatPhoto<'a> {
    pub(crate) fn new<C, P>(bot: &'a Bot, chat_id: C, photo: P) -> Self
    where
        C: Into<ChatId>,
        P: Into<InputFile>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            photo: photo.into(),
        }
    }

    pub fn chat_id<C>(mut self, chat_id: C) -> Self
    where
        C: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn photo<P>(mut self, photo: P) -> Self
    where
        P: Into<InputFile>,
    {
        self.photo = photo.into();
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
        let photo_url = "https://some_url".to_string();
        let method =
            SetChatPhoto::new(&bot, chat_id, InputFile::Url(photo_url));

        let expected = r#"{"chat_id":123,"photo":"https://some_url"}"#;
        let actual = serde_json::to_string::<SetChatPhoto>(&method).unwrap();
        assert_eq!(actual, expected);
    }
}
