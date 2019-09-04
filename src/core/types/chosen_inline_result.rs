use serde::Deserialize;

use crate::core::types::user::User;

#[derive(Debug, Deserialize)]
/// Represents a result of an inline query that was chosen by the user and
/// sent to their chat partner.
/// https://core.telegram.org/bots/api#inputtextmessagecontent
pub struct ChosenInlineResult {
    /// The unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Optional. Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Optional. Identifier of the sent inline message. Available only if there is an inline
    /// keyboard attached to the message. Will be also received in callback queries and can
    /// be used to edit the message.
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
