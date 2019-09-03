use reqwest::r#async::multipart::Form;

use crate::core::types::Message;

use super::form_builder::FormBuilder;
use super::{ChatId, Request};

#[derive(Debug, TypedBuilder, PartialEq, Eq)]
pub struct SendMessage {
    token: String,
    chat_id: ChatId,
    text: String,

    #[builder(default)]
    parse_mode: Option<String>,
    // TODO: enum
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

    fn name(&self) -> &str {
        "getMe"
    }
    fn params(self) -> Option<Form> {
        Some(
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .add("text", &self.text)
                .add_if_some("parse_mode", self.parse_mode.as_ref())
                .add_if_some(
                    "disable_web_page_preview",
                    self.disable_web_page_preview.as_ref(),
                )
                .add_if_some("disable_notification", self.disable_notification.as_ref())
                .add_if_some("reply_to_message_id", self.reply_to_message_id.as_ref())
                .build(),
        )

        // TODO:
        // .add_if_some("reply_markup",
        //              self.reply_markup.as_ref()))
    }
    fn token(&self) -> &str {
        &self.token
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default() {
        let sm = SendMessage::builder()
            .token("TOKEN")
            .chat_id(123456.into())
            .text("text")
            .build();
        let r = SendMessage {
            token: String::from("TOKEN"),
            chat_id: ChatId::Id(123456),
            text: String::from("text"),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        };

        assert_eq!(sm, r);
    }
}
