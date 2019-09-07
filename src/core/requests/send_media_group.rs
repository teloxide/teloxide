use crate::core::{
    types::{
        Message, InputMedia,
    },
    network::{
        request, ResponseResult,
    },
    requests::{
        form_builder::FormBuilder,
        ChatId,
        Request,
        RequestInfo,
        RequestFuture,
    }
};

/// Use this method to send a group of photos or videos as an album.
#[derive(Debug, TypedBuilder)]
pub struct SendMediaGroup {
    info: RequestInfo,

    chat_id: ChatId,
    media: Vec<InputMedia>,

    #[builder(default)]
    disable_notification: Option<bool>,
    #[builder(default)]
    reply_to_message_id: Option<i64>,
}

impl Request for SendMediaGroup {
    type ReturnValue = Vec<Message>;


    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
            let params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add("media", &self.media)
                .add_if_some("disable_notification", self.disable_notification.as_ref())
                .add_if_some("reply_to_message_id", self.reply_to_message_id.as_ref())
                .build();
            request(&self.info.client, &self.info.token, "sendMediaGroup", Some(params)).await
        })
    }
}
