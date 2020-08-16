use serde::{Deserialize, Serialize};

use crate::types::{Location, User};

/// Represents a [result] of an inline query that was chosen by the user and
/// sent to their chat partner.
///
/// [The official docs](https://core.telegram.org/bots/api#choseninlineresult).
///
/// [result]: https://core.telegram.org/bots/api#inlinequeryresult
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen.
    pub result_id: String,

    /// The user that chose the result.
    pub from: User,

    ///  A sender location, only for bots that require user location.
    pub location: Option<Location>,

    /// An identifier of the sent inline message. Available only if
    /// there is an [inline keyboard] attached to the message. Will be also
    /// received in [callback queries] and can be used to [edit] the message.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots/api#inlinekeyboardmarkup
    /// [callback queries]: https://core.telegram.org/bots/api#callbackquery
    /// [edit]: https://core.telegram.org/bots/api#updating-messages
    pub inline_message_id: Option<String>,

    /// The query that was used to obtain the result.
    pub query: String,
}

impl ChosenInlineResult {
    pub fn new<S1, S2>(result_id: S1, from: User, query: S2) -> Self
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        Self {
            result_id: result_id.into(),
            from,
            location: None,
            inline_message_id: None,
            query: query.into(),
        }
    }

    pub fn result_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.result_id = val.into();
        self
    }

    pub fn from(mut self, val: User) -> Self {
        self.from = val;
        self
    }

    pub fn location<S>(mut self, val: Location) -> Self {
        self.location = val.into();
        self
    }

    pub fn inline_message_id<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.inline_message_id = Some(val.into());
        self
    }

    pub fn query<S>(mut self, val: S) -> Self
    where
        S: Into<String>,
    {
        self.query = val.into();
        self
    }
}
