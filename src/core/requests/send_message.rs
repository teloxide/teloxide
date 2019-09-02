use super::{ChatId, Request};
use crate::core::other::Message;

use reqwest::r#async::multipart::Form;

#[derive(Debug, TypedBuilder, PartialEq, Eq)]
pub struct SendMessage {
    token: String,
    chat_id: ChatId,
    text: String,

    #[builder(default)]
    parse_mode: Option<String>, // TODO: enum
    #[builder(default)]
    disable_web_page_preview: Option<bool>,
    #[builder(default)]
    disable_notification: Option<bool>,
    #[builder(default)]
    reply_to_message_id: Option<i64>,
    #[builder(default)]
    reply_markup: Option<()>, // TODO: ReplyMarkup enum
}

impl Request<Message> for SendMessage {
    fn name(&self) -> &str {
        "getMe"
    }
    fn params(self) -> Option<Form> {
        use apply::Apply;

        // TODO: we need better serialization
        let params = Form::new()
            .text("chat_id", format!("{:?}", self.chat_id))
            .text("text", self.text)
            .apply(|f| {
                if let Some(parse_mode) = self.parse_mode {
                    f.text("parse_mode", parse_mode);
                    f
                } else {
                    f
                }
            })
            .apply(|f| {
                if let Some(disable_web_page_preview) = self.disable_web_page_preview {
                    f.text(
                        "disable_web_page_preview",
                        format!("{:?}", disable_web_page_preview),
                    );
                    f
                } else {
                    f
                }
            })
            .apply(|f| {
                if let Some(disable_notification) = self.disable_notification {
                    f.text(
                        "disable_notification",
                        format!("{:?}", disable_notification),
                    );
                    f
                } else {
                    f
                }
            })
            .apply(|f| {
                if let Some(reply_to_message_id) = self.reply_to_message_id {
                    f.text("reply_to_message_id", format!("{:?}", reply_to_message_id));
                    f
                } else {
                    f
                }
            })
            .apply(|f| {
                if let Some(reply_markup) = self.reply_markup {
                    unimplemented!();
                    //f.text("reply_markup", );
                    f
                } else {
                    f
                }
            });

        Some(params)
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
