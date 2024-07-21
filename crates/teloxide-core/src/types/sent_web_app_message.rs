use serde::{Deserialize, Serialize};

/// Contains information about an inline message sent by a [Web App] on behalf
/// of a user.
///
/// [Web App]: https://core.telegram.org/bots/webapps
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct SentWebAppMessage {
    /// Identifier of the sent inline message. Available only if there is an
    /// inline keyboard attached to the message.
    pub inline_message_id: Option<String>,
}
