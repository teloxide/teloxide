use serde::{Deserialize, Serialize};

/// Contains information about why a request was unsuccessful.
///
/// [The official docs](https://core.telegram.org/bots/api#responseparameters).
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseParameters {
    /// The group has been migrated to a supergroup with the specified
    /// identifier. This number may be greater than 32 bits and some
    /// programming languages may have difficulty/silent defects in
    /// interpreting it. But it is smaller than 52 bits, so a signed 64 bit
    /// integer or double-precision float type are safe for storing this
    /// identifier.
    MigrateToChatId(i64),

    /// In case of exceeding flood control, the number of seconds left to wait
    /// before the request can be repeated.
    RetryAfter(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_to_chat_id_deserialization() {
        let expected = ResponseParameters::MigrateToChatId(123_456);
        let actual: ResponseParameters =
            serde_json::from_str(r#"{"migrate_to_chat_id":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn retry_after_deserialization() {
        let expected = ResponseParameters::RetryAfter(123_456);
        let actual: ResponseParameters = serde_json::from_str(r#"{"retry_after":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }
}
