use serde::{Deserialize, Serialize};

use super::PassportFile;

/// Contains information about documents or other Telegram Passport elements
/// shared with the bot by the user.
///
/// [The official docs](https://core.telegram.org/bots/api#encryptedpassportelement).
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct EncryptedPassportElement {
    /// Base64-encoded element hash for using in
    /// [`PassportElementErrorUnspecified`].
    ///
    /// [`PassportElementErrorUnspecified`]:
    /// crate::types::PassportElementErrorUnspecified
    pub hash: String,

    #[serde(flatten)]
    pub kind: EncryptedPassportElementKind,
}

#[serde_with_macros::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
#[allow(clippy::large_enum_variant)]
pub enum EncryptedPassportElementKind {
    PersonalDetails {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,
    },
    Passport {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,

        /// Encrypted file with the front side of the document, provided by the
        /// user. Available for `passport”, `driver_license”, `identity_card”
        /// and `internal_passport”. The file can be decrypted and verified
        /// using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        front_side: PassportFile,

        /// Encrypted file with the selfie of the user holding a document,
        /// provided by the user; available for `passport”, `driver_license”,
        /// `identity_card” and `internal_passport”. The file can be decrypted
        /// and verified using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        selfie: PassportFile,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    DriverLicense {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,

        /// Encrypted file with the front side of the document, provided by the
        /// user. Available for `passport”, `driver_license”, `identity_card”
        /// and `internal_passport”. The file can be decrypted and verified
        /// using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        front_side: PassportFile,

        /// Encrypted file with the reverse side of the document, provided by
        /// the user. Available for `driver_license” and `identity_card”. The
        /// file can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        reverse_side: PassportFile,

        /// Encrypted file with the selfie of the user holding a document,
        /// provided by the user; available for `passport”, `driver_license”,
        /// `identity_card” and `internal_passport”. The file can be decrypted
        /// and verified using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        selfie: PassportFile,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    IdentityCard {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,

        /// Encrypted file with the front side of the document, provided by the
        /// user. Available for `passport”, `driver_license”, `identity_card”
        /// and `internal_passport”. The file can be decrypted and verified
        /// using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        front_side: PassportFile,

        /// Encrypted file with the reverse side of the document, provided by
        /// the user. Available for `driver_license” and `identity_card”. The
        /// file can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        reverse_side: PassportFile,

        /// Encrypted file with the selfie of the user holding a document,
        /// provided by the user; available for `passport”, `driver_license”,
        /// `identity_card” and `internal_passport”. The file can be decrypted
        /// and verified using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        selfie: PassportFile,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    InternalPassport {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,

        /// Encrypted file with the front side of the document, provided by the
        /// user. Available for `passport”, `driver_license”, `identity_card”
        /// and `internal_passport”. The file can be decrypted and verified
        /// using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        front_side: PassportFile,

        /// Encrypted file with the selfie of the user holding a document,
        /// provided by the user; available for `passport”, `driver_license”,
        /// `identity_card” and `internal_passport”. The file can be decrypted
        /// and verified using the accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        selfie: PassportFile,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    Address {
        ///  Base64-encoded encrypted Telegram Passport element data provided
        /// by the user, available for `personal_details”, `passport”,
        /// `driver_license”, `identity_card”, `internal_passport” and
        /// `address” types. Can be decrypted and verified using the
        /// accompanying [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        data: String,
    },
    UtilityBill {
        /// Array of encrypted files with documents provided by the user,
        /// available for `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        files: Vec<PassportFile>,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    BankStatement {
        /// Array of encrypted files with documents provided by the user,
        /// available for `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        files: Vec<PassportFile>,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    RentalAgreement {
        /// Array of encrypted files with documents provided by the user,
        /// available for `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        files: Vec<PassportFile>,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    PassportRegistration {
        /// Array of encrypted files with documents provided by the user,
        /// available for `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        files: Vec<PassportFile>,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    TemporaryRegistration {
        /// Array of encrypted files with documents provided by the user,
        /// available for `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        files: Vec<PassportFile>,

        /// Array of encrypted files with translated versions of documents
        /// provided by the user. Available if requested for `passport”,
        /// `driver_license”, `identity_card”, `internal_passport”,
        /// `utility_bill”, `bank_statement”, `rental_agreement”,
        /// `passport_registration” and `temporary_registration” types. Files
        /// can be decrypted and verified using the accompanying
        /// [`EncryptedCredentials`].
        ///
        /// [`EncryptedCredentials`]:
        /// crate::types::EncryptedCredentials
        translation: Option<Vec<PassportFile>>,
    },
    PhoneNumber {
        /// User's verified phone number, available only for `phone_number”
        /// type.
        phone_number: String,
    },
    Email {
        /// User's verified email address, available only for `email” type.
        email: String,
    },
}
