use crate::core::{
    types::{Message, InputMedia, InputFile},
    network::request_multipart,
    requests::{
        form_builder::FormBuilder,
        ChatId,
        Request,
        RequestContext,
        RequestFuture,
        ResponseResult,
    }
};
use apply::Apply;

/// Use this method to send a group of photos or videos as an album.
#[derive(Debug, Clone)]
pub struct SendMediaGroup<'a> {
    ctx: RequestContext<'a>,

    pub chat_id: ChatId,
    pub media: Vec<InputMedia>,

    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
}

impl<'a> Request<'a> for SendMediaGroup<'a> {
    type ReturnValue = Vec<Message>;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .apply(|form| {
                    self.media
                        .iter()
                        .map(|e| e.media())
                        .fold(form, |acc, file| {
                            if let InputFile::File(path) = file {
                                acc.add_file(
                                &path
                                    .file_name()
                                    .unwrap()
                                    .to_string_lossy(),
                            path
                                )
                            } else {
                                acc
                            }
                    })
                })
                .add("media", &self.media)
                .add_if_some("disable_notification", self.disable_notification.as_ref())
                .add_if_some("reply_to_message_id", self.reply_to_message_id.as_ref())
                .build();
            request_multipart(&self.ctx.client, &self.ctx.token, "sendMediaGroup", Some(params)).await
        })
    }
}

impl<'a> SendMediaGroup<'a> {
    pub(crate) fn new(
        ctx: RequestContext<'a>,
        chat_id: ChatId,
        media: Vec<InputMedia>,
    ) -> Self {
        SendMediaGroup {
            ctx,
            chat_id,
            media,
            disable_notification: None,
            reply_to_message_id: None,
        }
    }

    pub fn chat_id<T: Into<ChatId>>(mut self, val: T) -> Self {
        self.chat_id = val.into();
        self
    }

    pub fn media<T: Into<Vec<InputMedia>>>(mut self, val: T) -> Self {
        self.media = val.into();
        self
    }

    pub fn disable_notification<T: Into<bool>>(mut self, val: T) -> Self {
        self.disable_notification = Some(val.into());
        self
    }

    pub fn reply_to_message_id<T: Into<i64>>(mut self, val: T) -> Self {
        self.reply_to_message_id = Some(val.into());
        self
    }
}
