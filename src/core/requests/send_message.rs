use crate::core::{
    types::Message,
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


#[derive(Debug, TypedBuilder)]
pub struct SendMessage {
    info: RequestInfo,

    chat_id: ChatId,
    text: String,

    #[builder(default)]
    parse_mode: Option<String>, // TODO: ParseMode enum
    #[builder(default)]
    disable_web_page_preview: Option<bool>,
    #[builder(default)]
    disable_notification: Option<bool>,
    #[builder(default)]
    reply_to_message_id: Option<i64>,
    #[builder(default)]
    reply_markup: Option<()>, // TODO: ReplyMarkup enum
}

impl Request for SendMessage {
    type ReturnValue = Message;


    fn send(self) -> RequestFuture<ResponseResult<Self::ReturnValue>> {
        Box::new(async {
            let params = FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add("text", &self.text)
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some(
                    "disable_web_page_preview",
                    self.disable_web_page_preview.as_ref(),
                )
                .add_if_some("disable_notification", self.disable_notification.as_ref())
                .add_if_some("reply_to_message_id", self.reply_to_message_id.as_ref())
                .build();

            request(&self.info.client, &self.info.token, "sendMessage", Some(params)).await
        })
    }
}
