use serde::{Deserialize, Serialize};

/// A unique message identifier.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(#[serde(rename = "message_id")] pub i32);

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
