use serde::{Deserialize, Serialize};

use crate::types::{ChatType, Location, User};

/// This object represents an incoming inline query.
///
/// When the user sends an empty query, your bot could return some default or
/// trending results.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequery).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender.
    pub from: User,

    /// Sender location, only for bots that request user location.
    pub location: Option<Location>,

    /// Text of the query (up to 512 characters).
    pub query: String,

    /// Offset of the results to be returned, can be controlled by the bot.
    pub offset: String,

    /// Type of the chat, from which the inline query was sent.
    ///
    /// The chat type should be always known for requests sent from official
    /// clients and most third-party clients, unless the request was sent
    /// from a secret chat.
    pub chat_type: Option<ChatType>,
}

// TODO(waffle): remove
impl InlineQuery {
    pub fn new<S1, S2, S3>(id: S1, from: User, query: S2, offset: S3) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        Self {
            id: id.into(),
            from,
            location: None,
            query: query.into(),
            offset: offset.into(),
            chat_type: None,
        }
    }

    pub fn id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.id = val.into();
        self
    }

    pub fn from(mut self, val: User) -> Self {
        self.from = val;
        self
    }

    pub fn location(mut self, val: Location) -> Self {
        self.location = Some(val);
        self
    }

    pub fn query<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.query = val.into();
        self
    }

    pub fn offset<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.offset = val.into();
        self
    }
}
