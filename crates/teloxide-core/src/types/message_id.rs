use serde::{Deserialize, Serialize};

/// A unique message identifier.
#[derive(
    Default,
    Clone,
    Copy,
    Debug,
    derive_more::Display,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize
)]
#[serde(from = "MessageIdRaw", into = "MessageIdRaw")]
pub struct MessageId(pub i32);

// N.B. we [de]serialize `MessageId` as `{"message_id":n}`, which means that if
//      you want just an integer, you need to special case it with something
//      like `serde(with = "crate::types::option_msg_id_as_int")]`
//
//      (we can't change the default format of `MessageId` because it's returned
//      by some methods and we can't change serialization there)

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
