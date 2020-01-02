use serde::Serialize;

use crate::{
    network,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ReplyMarkup},
    Bot,
};

/// Use this method to send phone contacts. On success, the sent Message is
/// returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct SendContact<'a> {
    #[serde(skip_serializing)]
    bot: &'a Bot,

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format @channelusername)
    chat_id: ChatId,
    /// Contact's phone number
    phone_number: String,
    /// Contact's first name
    first_name: String,
    /// Contact's last name
    last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no
    /// sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline
    /// keyboard, custom reply keyboard, instructions to remove keyboard or to
    /// force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

#[async_trait::async_trait]
impl Request<Message> for SendContact<'_> {
    async fn send(&self) -> ResponseResult<Message> {
        network::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendContact",
            &serde_json::to_string(self).unwrap(),
        )
        .await
    }
}

impl<'a> SendContact<'a> {
    pub(crate) fn new<C, P, F>(
        bot: &'a Bot,
        chat_id: C,
        phone_number: P,
        first_name: F,
    ) -> Self
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

    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    pub fn phone_number<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.phone_number = val.into();
        self
    }

    pub fn first_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.first_name = val.into();
        self
    }

    pub fn last_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(val.into());
        self
    }

    pub fn vcard<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(val.into());
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.reply_markup = Some(val);
        self
    }
}
