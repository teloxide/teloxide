use derive_more::derive::From;
use serde::{Deserialize, Serialize};

use crate::types::{ChatType, Location, User};

/// Unique query identifier.
#[derive(
    Default,
    Clone,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    From
)]
#[serde(transparent)]
#[from(&'static str)]
pub struct InlineQueryId(pub String);

/// This object represents an incoming inline query.
///
/// When the user sends an empty query, your bot could return some default or
/// trending results.
///
/// [The official docs](https://core.telegram.org/bots/api#inlinequery).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct InlineQuery {
    /// Unique identifier for this query.
    pub id: InlineQueryId,

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
