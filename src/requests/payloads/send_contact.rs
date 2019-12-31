use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, ReplyMarkup, Message},
};

/// Use this method to send phone contacts. On success, the sent Message is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct SendContact {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Contact's phone number
    phone_number: String,
    /// Contact's first name
    first_name: String,
    /// Contact's last name
    last_name: Option<String>,
    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    vcard: Option<String>,
    /// Sends the message silently. Users will receive a notification with no sound.
    disable_notification: Option<bool>,
    /// If the message is a reply, ID of the original message
    reply_to_message_id: Option<i32>,
    /// Additional interface options. A JSON-serialized object for an inline keyboard, custom reply keyboard, instructions to remove keyboard or to force a reply from the user.
    reply_markup: Option<ReplyMarkup>,
}

impl Method for SendContact {
    type Output = Message;

    const NAME: &'static str = "sendContact";
}

impl json::Payload for SendContact {}

impl dynamic::Payload for SendContact {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl SendContact {
    pub fn new<C, P, F>(chat_id: C, phone_number: P, first_name: F) -> Self
    where
        C: Into<ChatId>,
        P: Into<String>,
        F: Into<String>
    {
        let chat_id = chat_id.into();
        let phone_number = phone_number.into();
        let first_name = first_name.into();
        Self {
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
}

impl json::Request<'_, SendContact> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn phone_number<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.phone_number = val.into();
        self
    }

    pub fn first_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.first_name = val.into();
        self
    }

    pub fn last_name<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.last_name = Some(val.into());
        self
    }

    pub fn vcard<T>(mut self, val: T) -> Self
    where
        T: Into<String>
    {
        self.payload.vcard = Some(val.into());
        self
    }

    pub fn disable_notification(mut self, val: bool) -> Self {
        self.payload.disable_notification = Some(val);
        self
    }

    pub fn reply_to_message_id(mut self, val: i32) -> Self {
        self.payload.reply_to_message_id = Some(val);
        self
    }

    pub fn reply_markup(mut self, val: ReplyMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
                 