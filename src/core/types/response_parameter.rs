#[derive(Debug, Deserialize, Hash, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ResponseParameters {
    MigrateToChatId(i64),
    RetryAfter(i64),
}

#[cfg(test)]
mod tests {
    #[test]
    fn migrate_to_chat_id_deserialization() {
        let expected_struct = ResponseParameters::MigrateToChatId(123456);
        let actual_json: ResponseParameters = serde_json::from_str(
            r#"{"migrate_to_chat_id":123456}"#
        ).unwrap();

        assert_eq!(expected_json, actual_json);
    }

    #[test]
    fn retry_after_deserialization() {
        let expected_struct = ResponseParameters::RetryAfter(123456);
        let actual_json: ResponseParameters = serde_json::from_str(
            r#"{"retry_after":123456}"#
        ).unwrap();

        assert_eq!(expected_json, actual_json);
    }
}