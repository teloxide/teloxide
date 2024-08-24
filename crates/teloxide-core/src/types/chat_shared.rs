use serde::{Deserialize, Serialize};

use crate::types::{ChatId, PhotoSize, RequestId};

/// Information about a chat that was shared with the bot using a
/// [`KeyboardButtonRequestChat`] button.
///
/// [`KeyboardButtonRequestChat`]: crate::types::KeyboardButtonRequestChat
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct ChatShared {
    /// Identifier of the request.
    pub request_id: RequestId,

    /// Identifier of the shared chat.
    pub chat_id: ChatId,

    /// Title of the chat, if it was requested.
    pub title: Option<String>,

    /// Username of the chat, if it was requested.
    pub username: Option<String>,

    /// Available sizes of the chat photo, if it was requested.
    pub photo: Option<Vec<PhotoSize>>,
}
