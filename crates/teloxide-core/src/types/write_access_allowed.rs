use serde::{Deserialize, Serialize};

/// This object represents a service message about a user allowing a bot to
/// write messages after adding the bot to the attachment menu or launching a
/// Web App from a link.
///
/// [The official docs](https://core.telegram.org/bots/api#writeaccessallowed).
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// Name of the Web App which was launched from a link
    pub web_app_name: Option<String>,
}
