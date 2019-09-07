use std::path::Path;

use crate::core::requests::{RequestContext, ChatId, Request, RequestFuture, ResponseResult};
use crate::core::types::{ParseMode, Message, InputFile};
use crate::core::requests::form_builder::FormBuilder;
use crate::core::network;

#[derive(Debug, Clone)]
pub struct SendPhoto<'a> {
    ctx: RequestContext<'a>,

    pub chat_id: ChatId,
    pub photo: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,

    // TODO: add reply_markup
}

impl<'a> Request<'a> for SendPhoto<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let mut params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add_if_some("caption", self.caption.as_ref())
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some(
                    "disable_notification",
                    self.disable_notification.as_ref()
                )
                .add_if_some(
                    "reply_to_message_id",
                    self.reply_to_message_id.as_ref()
                );
            
            params = match self.photo {
                InputFile::File(path) => params.add_file("photo", &path),
                InputFile::Url(url) => params.add("photo", &url),
                InputFile::FileId(file_id) => params.add("photo", &file_id),
            };
            let params = params.build();

            network::request(
                &self.ctx.client,
                &self.ctx.token,
                "sendPhoto",
                Some(params)
            ).await
        })
    }
}

impl<'a> SendPhoto<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        photo: InputFile
    ) -> Self {
        Self {
            ctx,
            chat_id,
            photo,
            caption: None,
            parse_mode: None,
            disable_notification: None,
            reply_to_message_id: None
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, chat_id: T) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    pub fn photo<T: Into<InputFile>>(mut self, photo: T) -> Self {
        self.photo = photo.into();
        self
    }

    pub fn caption<T: Into<String>>(mut self, caption: T) -> Self {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode<T: Into<ParseMode>>(mut self, parse_mode: T) -> Self {
        self.parse_mode = Some(parse_mode.into());
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, disable_notification: T) -> Self {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i64>>(mut self, reply_to_message_id: T) -> Self {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::r#async::Client;

    const TOKEN: &str = "882997251:AAGImZKe4cO6vDzluWzCgYqebziIMroN7uU";
    const USER_ID: i64 = 268486177;

    #[test]
    fn send_photo() {
        use futures::FutureExt;
        use futures::TryFutureExt;
        tokio::run(async_send_photo().boxed().unit_error().compat())
    }

    async fn async_send_photo() {
        let client = Client::new();
        let req = SendPhoto::new(
            RequestContext {
                client: &client,
                token: TOKEN,
            },
            ChatId::Id(USER_ID),
            InputFile::File("D:\\Снимок.png".to_string().parse().unwrap()),
        );

        println!("{:?}", req.send().await);
    }
}
