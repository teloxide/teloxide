use serde::{Deserialize, Serialize};

use crate::{
    requests::{dynamic, json, Method},
    types::{ChatId, True},
};


/// Use this method for your bot to leave a group, supergroup or channel. Returns True on success.
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, PartialEq, Eq, Hash, Clone, Deserialize, Serialize)]
pub struct LeaveChat {
    /// Unique identifier for the target chat or username of the target supergroup or channel (in the format @channelusername)
    chat_id: ChatId,
}

impl Method for LeaveChat {
    type Output = True;

    const NAME: &'static str = "leaveChat";
}

impl json::Payload for LeaveChat {}

impl dynamic::Payload for LeaveChat {
    fn kind(&self) -> dynamic::Kind {
        dynamic::Kind::Json(serde_json::to_string(self).unwrap())
    }
}

impl LeaveChat {
    pub fn new<C>(chat_id: C) -> Self
    where
        C: Into<ChatId>
    {
        let chat_id = chat_id.into();
        Self {
            chat_id,
        }
    }
}

impl json::Request<'_, LeaveChat> {
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>
    {
        self.payload.chat_id = val.into();
        self
    }
}
                 