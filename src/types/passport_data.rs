use super::encrypted_credintials::EncryptedCredentials;
use super::encrypted_passport_element::EncryptedPassportElement;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct PassportData {
    pub data: Vec<EncryptedPassportElement>,
    pub credentials: EncryptedCredentials,
}

#[cfg(test)]
mod tests {
    use super::super::{ElementType, PassportFile};
    use super::*;

    #[test]
    fn must_serialize_passport_data_to_json() {
        let expected_json = r#"{
            "data":
            [
                {
                "type":"passport_registration",
                "data":"somedata",
                "phone_number":"1313",
                "email":"someemail",
                "front_size":
                {
                  "file_id":"someId",
                  "file_size":13,
                  "file_date":13
                },
                }
            ],
            "credential":
            {
                "data":"someData",
                "hash":"1122",
                "secret":"secret"
            }
        }"#
        .replace("\n", "")
        .replace(" ", "");
        let passport_data = PassportData {
            data: vec![EncryptedPassportElement {
                element_type: ElementType::PassportRegistration,
                data: "somedata".to_string(),
                phone_number: "1313".to_string(),
                email: "someemail".to_string(),
                files: None,
                front_size: Some(PassportFile {
                    file_id: "someId".to_string(),
                    file_size: 13,
                    file_date: 13,
                }),
                reverse_side: None,
                selfie: None,
                translation: None,
            }],
            credentials: EncryptedCredentials {
                data: "someData".to_string(),
                hash: "1122".to_string(),
                secret: "secret".to_string(),
            },
        };

        let actual_json = serde_json::to_string(&passport_data).unwrap();
        assert_eq!(actual_json, expected_json)
    }
}
