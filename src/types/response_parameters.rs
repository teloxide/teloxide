#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseParameters {
    MigrateToChatId(i64),
    RetryAfter(i32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrate_to_chat_id_deserialization() {
        let expected = ResponseParameters::MigrateToChatId(123456);
        let actual: ResponseParameters =
            serde_json::from_str(r#"{"migrate_to_chat_id":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn retry_after_deserialization() {
        let expected = ResponseParameters::RetryAfter(123456);
        let actual: ResponseParameters =
            serde_json::from_str(r#"{"retry_after":123456}"#).unwrap();

        assert_eq!(expected, actual);
    }
}
