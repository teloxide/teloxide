#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct EncryptedCredentials {
    // TODO: check base64 type
    pub data: String,
    pub hash: String,
    pub secret: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_serialize_encrypted_credentials_to_json() {
        // given
        let expected_json = r#"
        {
            "data":"someData",
            "hash":"1122",
            "secret":"secret"
        }"#
        .replace("\n", "")
        .replace(" ", "");
        let encrypted_credentials = EncryptedCredentials {
            data: "someData".to_string(),
            hash: "1122".to_string(),
            secret: "secret".to_string(),
        };
        // when
        let actual_json =
            serde_json::to_string(&encrypted_credentials).unwrap();
        //then
        assert_eq!(actual_json, expected_json)
    }
}
