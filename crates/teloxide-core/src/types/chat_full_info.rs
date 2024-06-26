use serde::{Deserialize, Serialize};

// TODO: in the TBA7.3 the Chat will be splitted into Chat and ChatInfo
// Currently it's just a container for the some fields of the Chat struct
#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    // FIXME: better type for the unix timestamp?
    /// Expiration date of the emoji status of the chat or the other party in a
    /// private chat, in Unix time, if any
    pub emoji_status_expiration_date: Option<i64>,
}
