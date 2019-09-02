use serde::{Serialize, Deserialize};

type NullableVec<T> = Option<Vec<T>>;

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct PassportData {
    data: Vec<EncryptedPassportElement>,
    credential: EncryptedCredentials,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct EncryptedPassportElement {
    r#type: ElementType,
    data: String,
    phone_number: String,
    email: String,
    files: NullableVec<PassportFile>,
    front_size: Option<PassportFile>,
    reverse_side: Option<PassportFile>,
    selfie: Option<PassportFile>,
    translation: NullableVec<PassportFile>,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct EncryptedCredentials {
    // todo check base64 type
    data: String,
    hash: String,
    secret: String,
}

#[derive(Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq)]
struct PassportFile {
    file_id: String,
    file_size: u64,
    file_date: u64,
}

mod tests {
    use super::*;

    fn create_passport_data() -> PassportData {
        PassportData {
            data: vec![create_encrypted_passport_element()],
            credential: create_encrypted_credentials(),
        }
    }

    fn create_encrypted_passport_element() -> EncryptedPassportElement {
        EncryptedPassportElement {
            r#type: ElementType::PassportRegistration,
            data: "somedata".to_string(),
            phone_number: "1313".to_string(),
            email: "someemail".to_string(),
            files: None,
            front_size: Some(create_passport_file()),
            reverse_side: None,
            selfie: None,
            translation: None,
        }
    }

    fn create_passport_file() -> PassportFile {
        PassportFile {
            file_id: "someId".to_string(),
            file_size: 13,
            file_date: 13,
        }
    }

    fn create_encrypted_credentials() -> EncryptedCredentials {
        EncryptedCredentials {
            data: "someData".to_string(),
            hash: "1122".to_string(),
            secret: "secret".to_string(),
        }
    }

    #[test]
    fn should_serialize_encrypted_passport_element_to_json() {
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
        }"#.replace("\n", "").replace(" ", "");
        let passport_element = create_encrypted_passport_element();
        // when
        let actual_json = serde_json::to_string(&passport_element).unwrap();
        //then
        assert_eq!(actual_json, expected_json)
    }

    #[test]
    fn should_serialize_encrypted_credentials_to_json() {
        // given
        let expected_json = r#"
        {
            "data":"someData",
            "hash":"1122",
            "secret":"secret"
        }"#.replace("\n", "").replace(" ", "");
        let encrypted_credentials = create_encrypted_credentials();
        // when
        let actual_json = serde_json::to_string(&encrypted_credentials).unwrap();
        //then
        assert_eq!(actual_json, expected_json)
    }

    #[test]
    fn should_serialize_passport_data_to_json() {
        // given
        let expected_json = r#"{
            "data":
            [
                {
                "type":"passport_registration",
                "data":"somedata",
                "phone_number":"1313",
                "email":"someemail",
                "files":null,
                "front_size":
                {
                  "file_id":"someId",
                  "file_size":13,
                  "file_date":13
                },
                "reverse_side":null,
                "selfie":null,
                "translation":null
                }
            ],
            "credential":
            {
                "data":"someData",
                "hash":"1122",
                "secret":"secret"
            }
        }"#.replace("\n", "").replace(" ", "");
        let passport_data = create_passport_data();
        // when
        let actual_json = serde_json::to_string(&passport_data).unwrap();
        //then
        assert_eq!(actual_json, expected_json)
    }
}