use serde::{Deserialize, Serialize};

use crate::types::{Location, User};

/// Represents a [result] of an inline query that was chosen by the user and
/// sent to their chat partner.
///
/// [The official docs](https://core.telegram.org/bots/api#choseninlineresult).
///
/// [result]: https://core.telegram.org/bots/api#inlinequeryresult
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
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
