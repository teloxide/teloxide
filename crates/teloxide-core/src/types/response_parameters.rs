use serde::{Deserialize, Serialize};

use crate::types::{ChatId, Seconds};

/// Contains information about why a request was unsuccessful.
///
/// [The official docs](https://core.telegram.org/bots/api#responseparameters).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseParameters {
    /// The group has been migrated to a supergroup with the specified
    /// identifier.
    MigrateToChatId(ChatId),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    RetryAfter(Seconds),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_to_chat_id_deserialization() {
        let expected = ResponseParameters::MigrateToChatId(ChatId(123_456));
        let actual: ResponseParameters =
            serde_json::from_str(r#"{"migrate_to_chat_id":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn retry_after_deserialization() {
        let expected = ResponseParameters::RetryAfter(Seconds::from_seconds(123_456));
        let actual: ResponseParameters = serde_json::from_str(r#"{"retry_after":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }
}
