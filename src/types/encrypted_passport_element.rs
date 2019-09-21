use super::passport_file::PassportFile;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EncryptedPassportElement {
    pub hash: String,
    #[serde(flatten)]
    pub kind: EncryptedPassportElementKind
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum EncryptedPassportElementKind {
    PersonalDetails {
        data: String
    },
    Passport {
        data: String,
        front_side: PassportFile,
        selfie: PassportFile,
        translation: Vec<PassportFile>
    },
    DriverLicense {
        data: String,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
        translation: Vec<PassportFile>
    },
    IdentityCard {
        data: String,
        front_side: PassportFile,
        reverse_side: PassportFile,
        selfie: PassportFile,
        translation: Vec<PassportFile>
    },
    InternalPassport {
        data: String,
        front_side: PassportFile,
        selfie: PassportFile,
        translation: Vec<PassportFile>
    },
    Address {
        data: String
    },
    UtilityBill {
        files: Vec<PassportFile>,
        translation: Vec<PassportFile>
    },
    BankStatement {
        files: Vec<PassportFile>,
        translation: Vec<PassportFile>
    },
    RentalAgreement {
        files: Vec<PassportFile>,
        translation: Vec<PassportFile>
    },
    PassportRegistration {
        files: Vec<PassportFile>,
        translation: Vec<PassportFile>
    },
    TemporaryRegistration {
        files: Vec<PassportFile>,
        translation: Vec<PassportFile>
    },
    PhoneNumber { phone_number: String },
    Email { email: String }
}
