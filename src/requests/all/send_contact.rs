use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send phone contacts.
///
/// [The official docs](https://core.telegram.org/bots/api#sendcontact).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendContact {
    #[serde(skip_serializing)]
    bot: Bot,
    chat_id: ChatId,
    phone_number: String,
    first_name: String,
    last_name: Option<String>,
    vcard: Option<String>,
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i32>,
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request for SendContact {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(self.bot.client(), self.bot.token(), "sendContact", &self).await
    }
}

impl SendContact {
    pub(crate) fn new<C, P, F>(bot: Bot, chat_id: C, phone_number: P, first_name: F) -> Self
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>,
    {
        let chat_id = chat_id.into();
        let phone_number = phone_number.into();
        let first_name = first_name.into();
        Self {
            bot,
            chat_id,
            phone_number,
            first_name,
            last_name: None,
            vcard: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Contact's phone number.
    pub fn phone_number<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    /// Contact's first name.
    pub fn first_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    /// Contact's last name.
    pub fn last_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    /// Additional data about the contact in the form of a [vCard], 0-2048
    /// bytes.
    ///
    /// [vCard]: https://en.wikipedia.org/wiki/VCard
    pub fn vcard<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(val.into());
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    /// Additional interface options.
    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
