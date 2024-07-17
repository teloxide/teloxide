use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// TODO: in the TBA7.3 the Chat will be splitted into Chat and ChatInfo
// Currently it's just a container for the some fields of the Chat struct
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatFullInfo {
    /// Expiration date of the emoji status of the chat or the other party in a
    /// private chat, in Unix time, if any
    #[serde(default, with = "crate::types::serde_opt_date_from_unix_timestamp")]
    pub emoji_status_expiration_date: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chat_full_info_de() {
        assert_eq!(
            serde_json::from_str::<ChatFullInfo>("{}").unwrap(),
            ChatFullInfo { emoji_status_expiration_date: None }
        );
        assert_eq!(
            serde_json::from_str::<ChatFullInfo>(
                r#"{
           "emoji_status_expiration_date": 1720708004
        }"#
            )
            .unwrap(),
            ChatFullInfo { emoji_status_expiration_date: DateTime::from_timestamp(1720708004, 0) }
        );
    }
}
