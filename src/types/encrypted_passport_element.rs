use super::PassportFile;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EncryptedPassportElement {
    pub hash: String,
    #[serde(flatten)]
    pub kind: EncryptedPassportElementKind,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementKind {
    PersonalDetails {
        data: String,
    },
    Passport {
        data: String,
        front_side: PassportFile,
        selfie: PassportFile,
        translation: Option<Vec<PassportFile>>,
    },
    DriverLicense {
        data: String,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
        translation: Option<Vec<PassportFile>>,
    },
    IdentityCard {
        data: String,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
        translation: Option<Vec<PassportFile>>,
    },
    InternalPassport {
        data: String,
        front_side: PassportFile,
        selfie: PassportFile,
        translation: Option<Vec<PassportFile>>,
    },
    Address {
        data: String,
    },
    UtilityBill {
        files: Vec<PassportFile>,
        translation: Option<Vec<PassportFile>>,
    },
    BankStatement {
        files: Vec<PassportFile>,
        translation: Option<Vec<PassportFile>>,
    },
    RentalAgreement {
        files: Vec<PassportFile>,
        translation: Option<Vec<PassportFile>>,
    },
    PassportRegistration {
        files: Vec<PassportFile>,
        translation: Option<Vec<PassportFile>>,
    },
    TemporaryRegistration {
        files: Vec<PassportFile>,
        translation: Option<Vec<PassportFile>>,
    },
    PhoneNumber {
        phone_number: String,
    },
    Email {
        email: String,
    },
}
