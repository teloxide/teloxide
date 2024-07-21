use serde::{Deserialize, Serialize};

/// This object represents a service message about a user allowing a bot to
/// write messages after adding the bot to the attachment menu or launching a
/// Web App from a link.
///
/// [The official docs](https://core.telegram.org/bots/api#writeaccessallowed).
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct WriteAccessAllowed {
    /// Name of the Web App which was launched from a link
    pub web_app_name: Option<String>,
    /// `true`, if the access was granted after the user accepted an explicit
    /// request from a Web App sent by the method [requestWriteAccess](https://core.telegram.org/bots/webapps#initializing-mini-apps)
    pub from_request: Option<bool>,
    /// `true`, if the access was granted when the bot was added to the
    /// attachment or side menu
    pub from_attachment_menu: Option<bool>,
}
