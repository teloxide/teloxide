use reqwest::r#async::multipart::Form;

use crate::core::types::Message;

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
        use apply::Apply;

        // TODO: we need better serialization
        let params = Form::new()
            .text("chat_id", format!("{:?}", self.chat_id))
            .text("text", self.text)
            .apply(|f| {
                self.parse_mode
                    .map_or_else(|| f, |parse_mode| f.text("parse_mode", parse_mode))
            })
            .apply(|f| {
                self.disable_web_page_preview.map_or_else(
                    || f,
                    |disable_web_page_preview| {
                        f.text(
                            "disable_web_page_preview",
                            format!("{:?}", disable_web_page_preview),
                        )
                    },
                )
            })
            .apply(|f| {
                self.disable_notification.map_or_else(
                    || f,
                    |disable_notification| {
                        f.text(
                            "disable_notification",
                            format!("{:?}", disable_notification),
                        )
                    },
                )
            })
            .apply(|f| {
                self.reply_to_message_id.map_or_else(
                    || f,
                    |reply_to_message_id| {
                        f.text("reply_to_message_id", format!("{:?}", reply_to_message_id))
                    },
                )
            })
            .apply(|f| {
                self.reply_markup.map_or_else(
                    || f,
                    |reply_markup| {
                        unimplemented!();
                        //f.text("reply_markup", );
                        f
                    },
                )
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
