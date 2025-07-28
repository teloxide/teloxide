use serde::{Deserialize, Serialize};

use crate::types::{BusinessConnectionId, Chat, MessageId};

/// This object is received when messages are deleted from a connected business
/// account.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(test, derive(schemars::JsonSchema))]
pub struct BusinessMessagesDeleted {
    /// Unique identifier of the business connection.
    pub business_connection_id: BusinessConnectionId,

    /// Information about a chat in the business account. The bot may not have
    /// access to the chat or the corresponding user.
    pub chat: Chat,

    /// The list of identifiers of deleted messages in the chat of the business
    /// account.
    #[serde(with = "crate::types::vec_msg_id_as_vec_int")]
    #[cfg_attr(test, schemars(with = "Vec<i32>"))]
    pub message_ids: Vec<MessageId>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize() {
        let json = r#"{
            "business_connection_id": "123", 
            "chat": {
                "id": 0,
                "type": "private",
                "username": "username",
                "first_name": "Anon"
            },
            "message_ids": [1, 2, 3]
        }"#;

        let result: BusinessMessagesDeleted = serde_json::from_str(json).unwrap();

        assert_eq!(result.message_ids, vec![MessageId(1), MessageId(2), MessageId(3)]);
    }
}
