use super::passport_file::PassportFile;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
pub struct EncryptedPassportElement {
    #[serde(rename = "type")]
    pub element_type: ElementType,
    pub data: String,
    pub phone_number: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<PassportFile>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub front_size: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_side: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<PassportFile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub translation: Option<Vec<PassportFile>>,
}

#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ElementType {
    PersonalData,
    Passport,
    DriverLicense,
    IdentityCard,
    Address,
    UtilityBill,
    BankStatement,
    RentalAgreement,
    PassportRegistration,
    TemporaryRegistration,
    PhoneNumber,
    Email,
}

mod tests {
    #[test]
    fn must_serialize_encrypted_passport_element_to_json() {
        // given
        let expected_json = r#"
        {
            "type":"passport_registration",
            "data":"somedata",
            "phone_number":"1313",
            "email":"someemail",
            "files":null,
            "front_size":{"file_id":"someId","file_size":13,"file_date":13},
            "reverse_side":null,
            "selfie":null,
            "translation":null
        }"#
        .replace("\n", "")
        .replace(" ", "");

        let passport_element = EncryptedPassportElement {
            element_type: ElementType::PassportRegistration,
            data: "somedata".to_string(),
            phone_number: "1313".to_string(),
            email: "someemail".to_string(),
            files: None,
            front_size: Some(create_passport_file()),
            reverse_side: None,
            selfie: None,
            translation: None,
        };

        let actual_json = serde_json::to_string(&passport_element).unwrap();
        assert_eq!(actual_json, expected_json)
    }
}
