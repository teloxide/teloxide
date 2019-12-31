use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, InlineKeyboardMarkup, Poll},
};

/// Use this method to stop a poll which was sent by the bot. On success, the stopped Poll with the final results is returned.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct StopPoll {
    /// Unique identifier for the target chat or username of the target channel (in the format @channelusername)
    chat_id: ChatId,
    /// Identifier of the original message with the poll
    message_id: i32,
    /// A JSON-serialized object for a new message inline keyboard.
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl Method for StopPoll {
    type Output = Poll;

    const NAME: &'static str = "stopPoll";
}

impl json::Payload for StopPoll {}

impl dynamic::Payload for StopPoll {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl StopPoll {
    pub fn new<C>(chat_id: C, message_id: i32) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
            message_id,
            reply_markup: None,
        }
    }
}

impl json::Request<'_, StopPoll> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }

    pub fn message_id(mut self, val: i32) -> Self {
        self.payload.message_id = val;
        self
    }

    pub fn reply_markup(mut self, val: InlineKeyboardMarkup) -> Self {
        self.payload.reply_markup = Some(val);
        self
    }
}
                 