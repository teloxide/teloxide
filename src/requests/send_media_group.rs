use apply::Apply;

use async_trait::async_trait;

use crate::{
    network::request_multipart,
    requests::{
        form_builder::FormBuilder, Request, RequestContext, ResponseResult,
    },
    types::{ChatId, InputFile, InputMedia, Message},
};

/// Use this method to send a group of photos or videos as an album.
#[derive(Debug, Clone)]
pub struct SendMediaGroup<'a> {
    ctx: RequestContext<'a>,

    pub chat_id: ChatId,
    pub media: Vec<InputMedia>,

    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
}

#[async_trait]
impl Request for SendMediaGroup<'_> {
    type ReturnValue = Vec<Message>;

    async fn send_boxed(self) -> ResponseResult<Self::ReturnValue> {
        self.send().await
    }
}

impl SendMediaGroup<'_> {
    pub async fn send(self) -> ResponseResult<Vec<Message>> {
        let params = FormBuilder::new()
            .add("chat_id", &self.chat_id)
            .apply(|form| {
                self.media
                    .iter()
                    .map(|e| e.media())
                    .fold(form, |acc, file| {
                        if let InputFile::File(path) = file {
                            acc.add_file(
                                &path.file_name().unwrap().to_string_lossy(),
                                path,
                            )
                        } else {
                            acc
                        }
                    })
            })
            .add("media", &self.media)
            .add_if_some(
                "disable_notification",
                self.disable_notification.as_ref(),
            )
            .add_if_some(
                "reply_to_message_id",
                self.reply_to_message_id.as_ref(),
            )
            .build();
        request_multipart(
            &self.ctx.client,
            &self.ctx.token,
            "sendMediaGroup",
            Some(params),
        )
        .await
    }
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new<C, M>(
        ctx: RequestContext<'a>,
        chat_id: C,
        media: M,
    ) -> Self
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        SendMediaGroup {
            ctx,
            chat_id: chat_id.into(),
            media: media.into(),
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    pub fn chat_id<T>(mut self, chat_id: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = chat_id.into();
        self
    }

    pub fn media<T>(mut self, media: T) -> Self
    where
        T: Into<Vec<InputMedia>>,
    {
        self.media = media.into();
        self
    }

    pub fn disable_notification<T>(mut self, disable_notification: T) -> Self
    where
        T: Into<bool>,
    {
        self.disable_notification = Some(disable_notification.into());
        self
    }

    pub fn reply_to_message_id<T>(mut self, reply_to_message_id: T) -> Self
    where
        T: Into<i32>,
    {
        self.reply_to_message_id = Some(reply_to_message_id.into());
        self
    }
}
