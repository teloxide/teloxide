use crate::core::requests::form_builder::FormBuilder;
use crate::core::requests::{ChatId, Request, RequestFuture, RequestInfo, ResponseResult};
use crate::core::{network, types::Message};

#[derive(Debug, TypedBuilder)]
pub struct SendMessage<'a> {
    info: RequestInfo<'a>,

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

impl<'a> Request<'a> for SendMessage<'a> {
    type ReturnValue = Message;

    fn send(self) -> RequestFuture<'a, ResponseResult<Self::ReturnValue>> {
        Box::pin(async move {
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

            network::request(
                &self.info.client,
                &self.info.token,
                "sendMessage",
                Some(params),
            )
            .await
        })
    }
}
