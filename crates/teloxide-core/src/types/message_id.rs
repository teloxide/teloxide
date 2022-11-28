use serde::{Deserialize, Serialize};

/// A unique message identifier.
#[derive(Clone, Copy, Debug, derive_more::Display, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(from = "MessageIdRaw", into = "MessageIdRaw")]
pub struct MessageId(pub i32);

#[derive(Serialize, Deserialize)]
struct MessageIdRaw {
    message_id: i32,
}

impl From<MessageIdRaw> for MessageId {
    fn from(MessageIdRaw { message_id }: MessageIdRaw) -> Self {
        MessageId(message_id)
    }
}

impl From<MessageId> for MessageIdRaw {
    fn from(MessageId(message_id): MessageId) -> Self {
        MessageIdRaw { message_id }
    }
}

#[cfg(test)]
mod tests {
    use crate::types::MessageId;

    #[test]
    fn smoke_deser() {
        let json = r#"{"message_id":123}"#;
        let mid: MessageId = serde_json::from_str(json).unwrap();
        assert_eq!(mid, MessageId(123));
    }

    #[test]
    fn smoke_ser() {
        let mid: MessageId = MessageId(123);
        let json = serde_json::to_string(&mid).unwrap();
        assert_eq!(json, r#"{"message_id":123}"#);
    }
}
